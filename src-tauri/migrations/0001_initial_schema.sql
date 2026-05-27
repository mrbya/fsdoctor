CREATE TABLE app_meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    root_path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    manifest_format_version INTEGER NOT NULL
);

CREATE TABLE scans (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    kind TEXT NOT NULL,
    status TEXT NOT NULL,
    started_at TEXT NOT NULL,
    finished_at TEXT,
    total_dirs INTEGER NOT NULL DEFAULT 0,
    total_files INTEGER NOT NULL DEFAULT 0,
    total_symlinks INTEGER NOT NULL DEFAULT 0,
    total_other INTEGER NOT NULL DEFAULT 0,
    total_bytes INTEGER NOT NULL DEFAULT 0,
    hashed_files INTEGER NOT NULL DEFAULT 0,
    unreadable_entries INTEGER NOT NULL DEFAULT 0,
    changed_during_scan INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    FOREIGN KEY(project_id) REFERENCES projects(id)
);

CREATE TABLE manifest_entries (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    relative_path TEXT NOT NULL,
    entry_kind TEXT NOT NULL,
    size_bytes INTEGER,
    mtime_ns INTEGER,
    readonly INTEGER NOT NULL DEFAULT 0,
    hash_algo TEXT,
    hash BLOB,
    last_seen_scan_id INTEGER,
    status TEXT NOT NULL,
    error_message TEXT,
    FOREIGN KEY(project_id) REFERENCES projects(id),
    FOREIGN KEY(last_seen_scan_id) REFERENCES scans(id),
    UNIQUE(project_id, relative_path)
);

CREATE TABLE check_results (
    id INTEGER PRIMARY KEY,
    scan_id INTEGER NOT NULL,
    relative_path TEXT NOT NULL,
    result_kind TEXT NOT NULL,
    expected_entry_kind TEXT,
    actual_entry_kind TEXT,
    expected_size_bytes INTEGER,
    actual_size_bytes INTEGER,
    expected_hash BLOB,
    actual_hash BLOB,
    message TEXT,
    FOREIGN KEY(scan_id) REFERENCES scans(id)
);

CREATE INDEX idx_manifest_project_path
ON manifest_entries(project_id, relative_path);

CREATE INDEX idx_check_results_scan_result
ON check_results(scan_id, result_kind);
