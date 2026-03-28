use crate::error::AppError;
use crate::types::{RefKind, RefLabel};
use std::path::Path;

pub fn list_refs(repo_path: &Path) -> Result<Vec<RefLabel>, AppError> {
    let repo = super::open_repo(repo_path)?;
    let mut labels = Vec::new();

    let head_ref = repo.head_ref().ok().flatten();
    let head_name = head_ref.as_ref().map(|r| r.name().as_bstr().to_string());

    // Local branches
    let platform = repo
        .references()
        .map_err(|e| AppError::Git(e.to_string()))?;

    let local = platform
        .local_branches()
        .map_err(|e| AppError::Git(e.to_string()))?;

    for reference in local.flatten() {
        let full_name = reference.name().as_bstr().to_string();
        let short = full_name
            .strip_prefix("refs/heads/")
            .unwrap_or(&full_name)
            .to_string();
        let is_head = head_name.as_deref() == Some(&full_name);
        labels.push(RefLabel {
            name: short,
            kind: RefKind::LocalBranch,
            is_head,
        });
    }

    // Remote branches
    let platform = repo
        .references()
        .map_err(|e| AppError::Git(e.to_string()))?;

    let remote = platform
        .remote_branches()
        .map_err(|e| AppError::Git(e.to_string()))?;

    for reference in remote.flatten() {
        let full_name = reference.name().as_bstr().to_string();
        let short = full_name
            .strip_prefix("refs/remotes/")
            .unwrap_or(&full_name)
            .to_string();
        labels.push(RefLabel {
            name: short,
            kind: RefKind::RemoteBranch,
            is_head: false,
        });
    }

    // Tags
    let platform = repo
        .references()
        .map_err(|e| AppError::Git(e.to_string()))?;

    let tags = platform
        .tags()
        .map_err(|e| AppError::Git(e.to_string()))?;

    for reference in tags.flatten() {
        let full_name = reference.name().as_bstr().to_string();
        let short = full_name
            .strip_prefix("refs/tags/")
            .unwrap_or(&full_name)
            .to_string();
        labels.push(RefLabel {
            name: short,
            kind: RefKind::Tag,
            is_head: false,
        });
    }

    Ok(labels)
}

/// Build a map from commit OID -> list of ref labels pointing to it.
pub fn ref_map(
    repo_path: &Path,
) -> Result<std::collections::HashMap<String, Vec<RefLabel>>, AppError> {
    let repo = super::open_repo(repo_path)?;
    let mut map: std::collections::HashMap<String, Vec<RefLabel>> = std::collections::HashMap::new();

    let head_ref = repo.head_ref().ok().flatten();
    let head_name = head_ref.as_ref().map(|r| r.name().as_bstr().to_string());

    let references = repo
        .references()
        .map_err(|e| AppError::Git(e.to_string()))?;

    let all = references
        .all()
        .map_err(|e| AppError::Git(e.to_string()))?;

    for reference in all.flatten() {
        let full_name = reference.name().as_bstr().to_string();

        let (short, kind) = if let Some(name) = full_name.strip_prefix("refs/heads/") {
            (name.to_string(), RefKind::LocalBranch)
        } else if let Some(name) = full_name.strip_prefix("refs/remotes/") {
            (name.to_string(), RefKind::RemoteBranch)
        } else if let Some(name) = full_name.strip_prefix("refs/tags/") {
            (name.to_string(), RefKind::Tag)
        } else {
            continue;
        };

        let is_head = head_name.as_deref() == Some(&full_name);

        // Peel to commit to get the actual commit OID for annotated tags
        let target_id = reference
            .into_fully_peeled_id()
            .map(|id| id.to_string())
            .unwrap_or_default();

        if !target_id.is_empty() {
            map.entry(target_id).or_default().push(RefLabel {
                name: short,
                kind,
                is_head,
            });
        }
    }

    Ok(map)
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
    fn test_list_refs_finds_branches() {
        let path = fixture_path();
        let refs = list_refs(&path).unwrap();

        let local_branches: Vec<&RefLabel> = refs
            .iter()
            .filter(|r| matches!(r.kind, RefKind::LocalBranch))
            .collect();

        assert!(
            local_branches.iter().any(|b| b.name == "main"),
            "Should find 'main' branch"
        );
        assert!(
            local_branches.iter().any(|b| b.name == "feature-branch"),
            "Should find 'feature-branch'"
        );
    }

    #[test]
    fn test_list_refs_finds_tags() {
        let path = fixture_path();
        let refs = list_refs(&path).unwrap();

        let tags: Vec<&RefLabel> = refs
            .iter()
            .filter(|r| matches!(r.kind, RefKind::Tag))
            .collect();

        assert!(
            tags.iter().any(|t| t.name == "v0.1.0"),
            "Should find 'v0.1.0' tag"
        );
    }

    #[test]
    fn test_list_refs_head_is_marked() {
        let path = fixture_path();
        let refs = list_refs(&path).unwrap();

        let head_refs: Vec<&RefLabel> = refs.iter().filter(|r| r.is_head).collect();
        assert_eq!(head_refs.len(), 1, "Exactly one ref should be HEAD");
        assert_eq!(head_refs[0].name, "main", "HEAD should point to main");
    }

    #[test]
    fn test_ref_map_maps_oids_to_labels() {
        let path = fixture_path();
        let map = ref_map(&path).unwrap();

        assert!(!map.is_empty(), "Ref map should not be empty");

        // At least one OID should have a label
        let total_labels: usize = map.values().map(|v| v.len()).sum();
        assert!(total_labels >= 3, "Should have at least main, feature-branch, and v0.1.0");
    }
}
