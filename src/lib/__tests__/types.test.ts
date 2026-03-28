import { describe, it, expect } from "vitest";
import type {
  CommitNode,
  CommitGraph,
  DiffResult,
  RepoStatus,
  RefLabel,
} from "$lib/types";

describe("TypeScript types", () => {
  it("CommitNode can be constructed with all required fields", () => {
    const node: CommitNode = {
      oid: "a".repeat(40),
      short_oid: "a".repeat(7),
      message: "Initial commit",
      author_name: "Test User",
      author_email: "test@test.com",
      author_timestamp: 1711454400,
      parents: [],
      refs: [],
      column: 0,
      row: 0,
    };
    expect(node.oid).toHaveLength(40);
    expect(node.short_oid).toHaveLength(7);
    expect(node.parents).toEqual([]);
  });

  it("CommitGraph holds nodes and edges", () => {
    const graph: CommitGraph = {
      nodes: [],
      edges: [],
      total_count: 0,
    };
    expect(graph.nodes).toEqual([]);
    expect(graph.total_count).toBe(0);
  });

  it("DiffResult holds files and stats", () => {
    const diff: DiffResult = {
      files: [
        {
          path: "test.txt",
          status: "modified",
          hunks: [
            {
              header: "@@ -1,3 +1,3 @@",
              lines: [
                { kind: "context", content: "line 1", old_line_no: 1, new_line_no: 1 },
                { kind: "remove", content: "old", old_line_no: 2, new_line_no: null },
                { kind: "add", content: "new", old_line_no: null, new_line_no: 2 },
              ],
            },
          ],
          is_binary: false,
        },
      ],
      stats: { files_changed: 1, insertions: 1, deletions: 1 },
    };
    expect(diff.files).toHaveLength(1);
    expect(diff.stats.files_changed).toBe(1);
  });

  it("RepoStatus reflects repo state", () => {
    const status: RepoStatus = {
      staged: [{ path: "file.txt", status: "modified" }],
      unstaged: [],
      untracked: ["new.txt"],
      branch: "main",
      ahead: 1,
      behind: 0,
    };
    expect(status.branch).toBe("main");
    expect(status.staged).toHaveLength(1);
    expect(status.untracked).toContain("new.txt");
  });

  it("RefLabel kinds match expected values", () => {
    const local: RefLabel = { name: "main", kind: "localbranch", is_head: true };
    const remote: RefLabel = { name: "origin/main", kind: "remotebranch", is_head: false };
    const tag: RefLabel = { name: "v1.0", kind: "tag", is_head: false };

    expect(local.kind).toBe("localbranch");
    expect(remote.kind).toBe("remotebranch");
    expect(tag.kind).toBe("tag");
  });
});
