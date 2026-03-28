use crate::error::AppError;
use crate::types::{CommitGraph, CommitNode, GraphEdge};
use gix::bstr::ByteSlice;
use std::collections::HashMap;
use std::path::Path;

/// Build the commit graph with lane (column) assignment.
///
/// `skip` and `limit` support pagination for infinite scroll.
pub fn build_commit_graph(
    repo_path: &Path,
    skip: usize,
    limit: usize,
) -> Result<CommitGraph, AppError> {
    let repo = super::open_repo(repo_path)?;

    // Get all reference tips to start walking from
    let mut tips: Vec<gix::ObjectId> = Vec::new();

    // HEAD
    if let Ok(head) = repo.head_id() {
        tips.push(head.detach());
    }

    // All branch heads
    if let Ok(refs) = repo.references() {
        if let Ok(all) = refs.all() {
            for r in all.flatten() {
                if let Ok(id) = r.into_fully_peeled_id() {
                    tips.push(id.detach());
                }
            }
        }
    }

    tips.sort();
    tips.dedup();

    if tips.is_empty() {
        return Ok(CommitGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            total_count: 0,
        });
    }

    // Walk all commits in topological order
    let mut walk = repo
        .rev_walk(tips)
        .sorting(gix::revision::walk::Sorting::ByCommitTime(
            gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
        ))
        .all()
        .map_err(|e| AppError::Git(e.to_string()))?;

    // Collect all commits (we need full traversal for accurate total_count)
    let mut all_commits: Vec<CommitInfo> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    while let Some(Ok(info)) = walk.next() {
        let oid = info.id.to_string();
        if !seen.insert(oid.clone()) {
            continue;
        }

        let commit = info
            .id()
            .object()
            .map_err(|e| AppError::Git(e.to_string()))?
            .into_commit();

        let parents: Vec<String> = commit.parent_ids().map(|id| id.to_string()).collect();

        let author = commit.author().map_err(|e| AppError::Git(e.to_string()))?;

        let message = commit
            .message_raw_sloppy()
            .lines()
            .next()
            .unwrap_or_default()
            .to_str_lossy()
            .to_string();

        let author_name = author.name.to_str_lossy().to_string();
        let author_email = author.email.to_str_lossy().to_string();
        let author_timestamp = author.time.seconds;

        all_commits.push(CommitInfo {
            oid,
            parents,
            message,
            author_name,
            author_email,
            author_timestamp,
        });
    }

    let total_count = all_commits.len();

    // Get ref map for labeling
    let ref_map = super::refs::ref_map(repo_path).unwrap_or_default();

    // Apply pagination
    let page: Vec<&CommitInfo> = all_commits.iter().skip(skip).take(limit).collect();

    // Assign lanes (columns) for the visible page
    let (nodes, edges) = assign_lanes(&page, &ref_map);

    Ok(CommitGraph {
        nodes,
        edges,
        total_count,
    })
}

struct CommitInfo {
    oid: String,
    parents: Vec<String>,
    message: String,
    author_name: String,
    author_email: String,
    author_timestamp: i64,
}

/// Lane assignment algorithm for commit graph visualization.
///
/// Assigns each commit to a column (lane) for visual layout.
/// The algorithm walks commits in topological order and:
/// - Inherits a lane from the first child that points to this commit
/// - Creates new lanes for merge parents
/// - Frees lanes when branches end
fn assign_lanes(
    commits: &[&CommitInfo],
    ref_map: &HashMap<String, Vec<crate::types::RefLabel>>,
) -> (Vec<CommitNode>, Vec<GraphEdge>) {
    let mut nodes = Vec::with_capacity(commits.len());
    let mut edges = Vec::new();

    // Active lanes: each slot holds the OID of the next expected commit in that lane
    let mut lanes: Vec<Option<String>> = Vec::new();
    let mut color_counter: u8 = 0;
    // Map from OID -> assigned lane color
    let mut lane_colors: HashMap<String, u8> = HashMap::new();

    for (row, commit) in commits.iter().enumerate() {
        // Find which lane this commit occupies
        let column = if let Some(pos) = lanes.iter().position(|l| l.as_deref() == Some(&commit.oid))
        {
            pos
        } else {
            // No lane reserved, allocate a new one
            let pos = lanes
                .iter()
                .position(|l| l.is_none())
                .unwrap_or_else(|| {
                    lanes.push(None);
                    lanes.len() - 1
                });
            lanes[pos] = Some(commit.oid.clone());
            pos
        };

        // Assign color if not already set
        let color = *lane_colors.entry(commit.oid.clone()).or_insert_with(|| {
            let c = color_counter;
            color_counter = (color_counter + 1) % 8;
            c
        });

        // Process parents
        let first_parent = commit.parents.first();

        // First parent inherits this lane
        if let Some(parent_oid) = first_parent {
            lanes[column] = Some(parent_oid.clone());
            lane_colors.entry(parent_oid.clone()).or_insert(color);

            edges.push(GraphEdge {
                from_oid: commit.oid.clone(),
                to_oid: parent_oid.clone(),
                color_index: color,
            });
        } else {
            // Root commit, free the lane
            lanes[column] = None;
        }

        // Additional parents (merge) get new lanes
        for parent_oid in commit.parents.iter().skip(1) {
            let parent_lane =
                if let Some(pos) = lanes.iter().position(|l| l.as_deref() == Some(parent_oid)) {
                    pos
                } else {
                    let pos = lanes
                        .iter()
                        .position(|l| l.is_none())
                        .unwrap_or_else(|| {
                            lanes.push(None);
                            lanes.len() - 1
                        });
                    lanes[pos] = Some(parent_oid.clone());
                    pos
                };

            let merge_color = *lane_colors.entry(parent_oid.clone()).or_insert_with(|| {
                let c = color_counter;
                color_counter = (color_counter + 1) % 8;
                c
            });

            let _ = parent_lane; // lane position used for visual placement

            edges.push(GraphEdge {
                from_oid: commit.oid.clone(),
                to_oid: parent_oid.clone(),
                color_index: merge_color,
            });
        }

        let refs = ref_map.get(&commit.oid).cloned().unwrap_or_default();

        nodes.push(CommitNode {
            oid: commit.oid.clone(),
            short_oid: commit.oid[..7.min(commit.oid.len())].to_string(),
            message: commit.message.clone(),
            author_name: commit.author_name.clone(),
            author_email: commit.author_email.clone(),
            author_timestamp: commit.author_timestamp,
            parents: commit.parents.clone(),
            refs,
            column: column as u32,
            row: row as u32,
        });
    }

    (nodes, edges)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("tests")
            .join("fixtures")
            .join("test-repo")
    }

    #[test]
    fn test_build_commit_graph_returns_commits() {
        let path = fixture_path();
        let graph = build_commit_graph(&path, 0, 100).unwrap();

        // We created 6 commits in the fixture (initial, file1, feature, file2, merge, post-merge)
        assert_eq!(graph.total_count, 6, "Expected 6 commits in fixture repo");
        assert_eq!(graph.nodes.len(), 6, "All 6 should be in the first page");
    }

    #[test]
    fn test_commit_graph_pagination() {
        let path = fixture_path();

        // Get first 3 commits
        let page1 = build_commit_graph(&path, 0, 3).unwrap();
        assert_eq!(page1.nodes.len(), 3);
        assert_eq!(page1.total_count, 6);

        // Get next 3 commits
        let page2 = build_commit_graph(&path, 3, 3).unwrap();
        assert_eq!(page2.nodes.len(), 3);

        // Verify no overlap
        let oids1: Vec<&str> = page1.nodes.iter().map(|n| n.oid.as_str()).collect();
        let oids2: Vec<&str> = page2.nodes.iter().map(|n| n.oid.as_str()).collect();
        for oid in &oids2 {
            assert!(!oids1.contains(oid), "Pages should not overlap");
        }
    }

    #[test]
    fn test_commit_graph_has_edges() {
        let path = fixture_path();
        let graph = build_commit_graph(&path, 0, 100).unwrap();

        // Every non-root commit should have at least one edge
        assert!(!graph.edges.is_empty(), "Graph should have edges");

        // The merge commit should create an extra edge
        let merge_commit = graph.nodes.iter().find(|n| n.message == "Merge feature-branch");
        assert!(merge_commit.is_some(), "Should find merge commit");
        let merge = merge_commit.unwrap();
        assert_eq!(merge.parents.len(), 2, "Merge commit should have 2 parents");
    }

    #[test]
    fn test_commit_graph_has_ref_labels() {
        let path = fixture_path();
        let graph = build_commit_graph(&path, 0, 100).unwrap();

        // HEAD/main should point to the latest commit
        let head_commit = graph
            .nodes
            .iter()
            .find(|n| n.refs.iter().any(|r| r.is_head));
        assert!(head_commit.is_some(), "Should find HEAD commit");

        // Tag v0.1.0 should exist
        let tagged = graph
            .nodes
            .iter()
            .find(|n| n.refs.iter().any(|r| r.name == "v0.1.0"));
        assert!(tagged.is_some(), "Should find tagged commit");
    }

    #[test]
    fn test_commit_graph_lane_assignment() {
        let path = fixture_path();
        let graph = build_commit_graph(&path, 0, 100).unwrap();

        // All nodes should have valid column assignments
        for node in &graph.nodes {
            // Columns should be small (we only have 2 lanes at most)
            assert!(
                node.column < 10,
                "Column {} is unreasonably large for a small repo",
                node.column
            );
        }

        // Row indices should be sequential
        for (i, node) in graph.nodes.iter().enumerate() {
            assert_eq!(node.row, i as u32, "Row should match index");
        }
    }

    #[test]
    fn test_commit_node_fields() {
        let path = fixture_path();
        let graph = build_commit_graph(&path, 0, 1).unwrap();
        let node = &graph.nodes[0];

        assert_eq!(node.oid.len(), 40, "OID should be 40 hex chars");
        assert_eq!(node.short_oid.len(), 7, "Short OID should be 7 chars");
        assert_eq!(node.author_name, "Test User");
        assert_eq!(node.author_email, "test@test.com");
        assert!(node.author_timestamp > 0, "Timestamp should be positive");
        assert!(!node.message.is_empty(), "Message should not be empty");
    }
}
