# Data model

This document defines the initial SQLite data model for `FSDoctor`.

The database is a project file.
It stores project metadata, scan history, the latest manifest, and generated check results.

## Design goals

The data model must support:

- opening an existing project database;
- generating and replacing a manifest;
- preserving scan history;
- recording incomplete or cancelled jobs;
- storing detailed per-path check results;
- exporting reports without rescanning the filesystem;
- future parity metadata without redesigning the project format.

## Database file

Recommended project file extension:

```text
.fsdoctor.sqlite
```

Example:

```text
FamilyArchive.fsdoctor.sqlite
```

The database should normally be stored outside the backup root.
The application should warn if the selected database location is inside the backup root.

## Schema-versioning strategy

The schema must be versioned by migrations.

The database should also include application-level metadata for compatibility checks.

Recommended metadata:

- schema format version;
- application version that created the project;
- application version that last updated the project;
- project creation timestamp.

## Initial tables

### `app_meta`

Stores key-value metadata for database/project compatibility.

```sql
CREATE TABLE app_meta (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

Suggested keys:

```text
schema_version
manifest_format_version
created_by_version
last_updated_by_version
created_at
```

### `projects`

Stores project-level metadata.

```sql
CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    root_path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    manifest_format_version INTEGER NOT NULL
);
```

Only one project row is expected in an MVP database.
The separate table still makes the model explicit and leaves room for future multi-root or multi-project support.

Fields:

| Field | Meaning |
| --- | --- |
| `id` | Internal project identifier. |
| `name` | User-facing project name. |
| `root_path` | Original selected backup root path. |
| `created_at` | Project creation timestamp. |
| `updated_at` | Last project update timestamp. |
| `manifest_format_version` | Version of the manifest semantics. |

### `scans`

Stores durable records for manifest-generation and integrity-check jobs.

```sql
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
```

Fields:

| Field | Meaning |
| --- | --- |
| `kind` | `manifest_generation` or `integrity_check`. |
| `status` | `running`, `completed`, `cancelled`, or `failed`. |
| `started_at` | Job start timestamp. |
| `finished_at` | Job finish timestamp if available. |
| `total_*` | Summary counters. |
| `hashed_files` | Number of regular files hashed. |
| `unreadable_entries` | Number of entries that could not be read or inspected. |
| `changed_during_scan` | Files whose metadata changed while being processed. |
| `error_message` | Job-level failure description, if any. |

### `manifest_entries`

Stores the current manifest state.

```sql
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
    last_seen_scan_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    error_message TEXT,
    FOREIGN KEY(project_id) REFERENCES projects(id),
    FOREIGN KEY(last_seen_scan_id) REFERENCES scans(id),
    UNIQUE(project_id, relative_path)
);
```

Fields:

| Field | Meaning |
| --- | --- |
| `relative_path` | Root-relative path and primary user-facing identity. |
| `entry_kind` | `file`, `directory`, `symlink`, or `other`. |
| `size_bytes` | File size for regular files, when available. |
| `mtime_ns` | Last-modified timestamp in nanoseconds where available. |
| `readonly` | Whether the entry was readonly at scan time. |
| `hash_algo` | Hash algorithm for hashed regular files. Initially `blake3`. |
| `hash` | Binary hash digest. |
| `last_seen_scan_id` | Manifest-generation scan that produced or updated this entry. |
| `status` | Manifest status such as `hashed`, `recorded`, `skipped`, or `unreadable`. |
| `error_message` | Entry-level issue, when present. |

For regular hashed files:

- `entry_kind = 'file'`
- `hash_algo = 'blake3'`
- `hash` is a 32-byte BLOB

For directories:

- `entry_kind = 'directory'`
- `hash_algo IS NULL`
- `hash IS NULL`

For skipped symlinks or special entries:

- `entry_kind` describes the detected type;
- `status` explains how it was handled;
- `hash` is usually `NULL`.

### `check_results`

Stores detailed results from integrity checks.

```sql
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
```

Fields:

| Field | Meaning |
| --- | --- |
| `scan_id` | Integrity-check scan that produced this result. |
| `relative_path` | Path being checked. |
| `result_kind` | Result classification. |
| `expected_*` | Values from the manifest. |
| `actual_*` | Values observed during the check. |
| `message` | Optional human-readable diagnostic. |

The table intentionally stores expected and actual hash values for detailed diagnostics and CSV export.
For `ok` entries, storing both may be redundant but keeps export logic simple.

## Indexes

Recommended MVP indexes:

```sql
CREATE INDEX idx_manifest_project_path
ON manifest_entries(project_id, relative_path);

CREATE INDEX idx_check_results_scan_result
ON check_results(scan_id, result_kind);

CREATE INDEX idx_check_results_scan_path
ON check_results(scan_id, relative_path);

CREATE INDEX idx_scans_project_started
ON scans(project_id, started_at);
```

## Enum values

SQLite stores enum-like values as text.
Rust code should model them as typed enums and serialize them explicitly.

### Scan kind

```text
manifest_generation
integrity_check
```

### Scan status

```text
running
completed
cancelled
failed
```

### Entry kind

```text
file
directory
symlink
other
```

### Manifest entry status

```text
hashed
recorded
skipped
unreadable
changed_during_scan
```

Meanings:

| Status | Meaning |
| --- | --- |
| `hashed` | Regular file was successfully hashed. |
| `recorded` | Non-file entry was recorded without hashing. |
| `skipped` | Entry was intentionally skipped, such as a symlink. |
| `unreadable` | Entry could not be inspected or read. |
| `changed_during_scan` | File metadata changed while being processed. |

### Check result kind

```text
ok
missing
new
hash_mismatch
size_mismatch
type_changed
unreadable
changed_during_check
skipped
```

Meanings:

| Result | Meaning |
| --- | --- |
| `ok` | Current entry matches manifest expectations. |
| `missing` | Manifest entry is absent from current tree. |
| `new` | Current entry is absent from manifest. |
| `hash_mismatch` | Size matched but content hash differed. |
| `size_mismatch` | Regular file size differed. |
| `type_changed` | Entry kind changed. |
| `unreadable` | Current entry could not be checked. |
| `changed_during_check` | File changed while being checked. |
| `skipped` | Entry was skipped by policy. |

## Path representation

The MVP stores root-relative paths as UTF-8 `TEXT`.

This is pragmatic for the Windows-first target.
Future versions may add a more exact representation for Unix paths that are not valid UTF-8.

Path rules:

- store paths relative to the backup root;
- do not store absolute paths as manifest identity;
- use a stable separator representation in the database;
- preserve display-friendly paths for reports;
- avoid string concatenation for filesystem operations.

## Timestamp representation

The MVP stores modification time as an integer nanosecond value where available.

Timestamp precision differs between filesystems and platforms.
Timestamp values are useful for detecting possible changes during a scan, but content hash is the authoritative file-content integrity value.

## Hash representation

Hashes are stored as binary BLOBs.

Initial algorithm:

```text
blake3
```

BLAKE3 output size:

```text
32 bytes
```

UI and CSV exports should convert hashes to lowercase hexadecimal strings.

## Report summaries

The `scans` table stores enough counters for summary cards.
Detailed per-path check results live in `check_results`.

If summary performance becomes a problem later, a dedicated summary table may be added.
For MVP, summaries can be derived from `scans` and aggregate queries over `check_results`.

## Future parity tables

Future PAR2 support should use separate tables rather than modifying manifest semantics.

Possible future tables:

```sql
CREATE TABLE parity_sets (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    root_relative_path TEXT NOT NULL,
    parity_path TEXT NOT NULL,
    backend TEXT NOT NULL,
    redundancy_percent INTEGER,
    created_at TEXT NOT NULL,
    status TEXT NOT NULL,
    FOREIGN KEY(project_id) REFERENCES projects(id)
);

CREATE TABLE parity_set_entries (
    id INTEGER PRIMARY KEY,
    parity_set_id INTEGER NOT NULL,
    relative_path TEXT NOT NULL,
    FOREIGN KEY(parity_set_id) REFERENCES parity_sets(id)
);
```

These are intentionally post-MVP.

## Data model acceptance criteria

The data model is accepted when:

- project metadata can be stored and loaded;
- manifest-generation scans are persisted;
- integrity-check scans are persisted;
- manifest entries are uniquely addressed by project and relative path;
- check results can be queried by scan, path, and result kind;
- CSV export can be produced from persisted records;
- cancelled and failed scans can be represented;
- the model does not require PAR2 tables for MVP work.
