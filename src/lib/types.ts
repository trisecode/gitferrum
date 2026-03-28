export interface CommitNode {
  oid: string;
  short_oid: string;
  message: string;
  author_name: string;
  author_email: string;
  author_timestamp: number;
  parents: string[];
  refs: RefLabel[];
  column: number;
  row: number;
}

export interface RefLabel {
  name: string;
  kind: RefKind;
  is_head: boolean;
}

export type RefKind = "localbranch" | "remotebranch" | "tag";

export interface CommitGraph {
  nodes: CommitNode[];
  edges: GraphEdge[];
  total_count: number;
}

export interface GraphEdge {
  from_oid: string;
  to_oid: string;
  color_index: number;
}

export interface DiffResult {
  files: FileDiff[];
  stats: DiffStats;
}

export interface FileDiff {
  path: string;
  status: FileStatus;
  hunks: DiffHunk[];
  is_binary: boolean;
}

export type FileStatus = "added" | "modified" | "deleted" | "renamed" | "copied";

export interface DiffHunk {
  header: string;
  lines: DiffLine[];
}

export interface DiffLine {
  kind: LineKind;
  content: string;
  old_line_no: number | null;
  new_line_no: number | null;
}

export type LineKind = "add" | "remove" | "context";

export interface DiffStats {
  files_changed: number;
  insertions: number;
  deletions: number;
}

export interface RepoStatus {
  staged: StatusEntry[];
  unstaged: StatusEntry[];
  untracked: string[];
  branch: string | null;
  ahead: number;
  behind: number;
}

export interface StatusEntry {
  path: string;
  status: FileStatus;
}

export interface PushStatus {
  has_upstream: boolean;
  has_remote: boolean;
  branch_name: string;
  remote_name: string;
}

export interface PullResult {
  success: boolean;
  message: string;
  has_conflicts: boolean;
  conflict_files: string[];
}
