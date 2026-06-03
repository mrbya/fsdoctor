# FSDoctor Implementation Roadmap

**App name:** FSDoctor
**Purpose:** Cross-platform backup/file-share integrity checker for non-technical users
**Target stack:** Tauri 2 + Rust backend + Svelte/TypeScript frontend
**Database:** SQLite, likely via SQLx
**Hashing:** BLAKE3
**UI style:** Minimalistic, Catppuccin Mocha, JetBrains Mono Nerd Font
**Primary target user:** Non-technical Windows user

---

## 1. Product definition

FSDoctor is a desktop application that creates and verifies file integrity manifests for large backup/file-share trees.

The app should allow a user to:

- Create an integrity record for a selected backup folder.
- Store the integrity data in a portable `.fsdoctor.sqlite` project database.
- Re-check the backup later against the stored manifest.
- Identify missing, changed, corrupt, unreadable, new, and skipped files.
- Display clear human-readable reports.
- Export CSV reports.
- Eventually create, verify, and repair PAR2 parity sets for selected parts of the tree.

The main product rule:

> The user should never need to understand hashes, SQLite, BLAKE3, or PAR2 to use the app.

Preferred user-facing language:

```text
Backup appears healthy.
No corrupted files were found.
3 files could not be checked.
22 new files were found.
```

Avoid making the app feel like a developer tool. The backend can be technical; the UI should be calm, direct, and diagnostic.

---

## 2. MVP definition

The MVP is complete when FSDoctor can:

- Create a new `.fsdoctor.sqlite` project database.
- Open an existing FSDoctor project database.
- Select a backup root directory.
- Generate a per-file BLAKE3 manifest.
- Store manifest entries in SQLite.
- Show scan progress while generating the manifest.
- Check the backup folder against the stored manifest.
- Produce an integrity report with precise per-file results.
- Export manifest-generation and integrity-check reports as CSV.
- Support cancellation of long-running jobs.
- Treat unreadable files as report entries instead of fatal errors.
- Handle Windows path and filesystem edge cases well enough for real-world backup drives.

Explicitly **not MVP**:

- PAR2 creation/verification/repair.
- Scheduled checks.
- Disk SMART/disk-health checks.
- Cloud sync.
- Automatic backup creation.
- Deduplication.
- Multi-backup projects.
- Deep analytics.
- Background service/daemon mode.

---

## 3. Architecture summary

FSDoctor should be built as a reusable Rust integrity engine plus a thin Tauri shell.

```text
FSDoctor
│
├── Svelte/TypeScript UI
│   ├── minimal Catppuccin Mocha interface
│   ├── project creation/opening workflow
│   ├── scan/check progress views
│   ├── report history and report detail views
│   └── CSV export actions
│
├── Tauri app layer
│   ├── commands
│   ├── event emission
│   ├── job registry
│   ├── cancellation
│   ├── app state
│   └── file/folder dialogs
│
└── fsdoctor-core Rust crate
    ├── filesystem walking
    ├── BLAKE3 hashing
    ├── SQLite project database
    ├── manifest generation
    ├── integrity checking
    ├── report generation
    ├── CSV export
    └── future parity abstraction
```

Important boundary:

```text
fsdoctor-core:
  no Tauri dependency
  testable from cargo test
  owns integrity logic

src-tauri/src:
  thin command layer
  job orchestration
  frontend event emission
```

---

## 4. Proposed repository layout

```text
fsdoctor/
├── src/                         # Svelte frontend
│   ├── lib/
│   │   ├── api/
│   │   ├── components/
│   │   ├── stores/
│   │   ├── theme/
│   │   └── views/
│   ├── routes/ or app shell
│   └── app.css
│
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs          # Thin Tauri command layer
│   │   ├── events.rs            # Progress/report event mapping
│   │   ├── jobs.rs              # Job registry/cancellation
│   │   └── state.rs             # AppState
│   │
│   ├── crates/
│   │   └── fsdoctor-core/
│   │       ├── src/
│   │       │   ├── lib.rs
│   │       │   ├── db/
│   │       │   ├── fs/
│   │       │   ├── hashing/
│   │       │   ├── manifest/
│   │       │   ├── check/
│   │       │   ├── report/
│   │       │   ├── csv/
│   │       │   ├── jobs/
│   │       │   └── parity/       # Stub for post-MVP
│   │       └── tests/
│   │
│   ├── migrations/
│   ├── capabilities/
│   └── tauri.conf.json
│
├── Cargo.toml                   # Optional workspace root
├── package.json
└── README.md
```

---

# 5. Phase roadmap

## Phase 0 — Product and technical foundation

### Goal

Lock down the MVP behavior, project boundaries, data model assumptions, and user workflows before implementing heavy scanning logic.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 0.1 | Define MVP scope | `docs/mvp-scope.md` |
| 0.2 | Define result taxonomy | `docs/result-kinds.md` |
| 0.3 | Define project database concept | `docs/project-db.md` |
| 0.4 | Define UX vocabulary | `docs/ux-language.md` |
| 0.5 | Decide path handling policy | `docs/path-policy.md` |

### Key decisions

- FSDoctor project files are SQLite databases with extension `.fsdoctor.sqlite`.
- The database should be stored outside the checked backup root by default.
- Paths stored in the manifest are root-relative.
- Absolute root path is stored as project metadata but is not the identity of each entry.
- Symlinks, junctions, and reparse points are not followed in the MVP.
- Unreadable files are reported, not fatal.
- BLAKE3 hashes are stored as binary BLOBs, not hex text.
- Hex strings are generated only for display/CSV export.

### Acceptance criteria

- [x] MVP scope is documented and explicitly excludes PAR2/repair.
- [x] Result kinds are documented with user-facing meaning.
- [x] Project DB lifecycle is documented: create, open, update, scan history.
- [x] Path policy is documented, including Windows-specific behavior.
- [x] UI vocabulary avoids exposing unnecessary implementation details.
- [x] The project has a written rule that the core crate must not depend on Tauri.

---

## Phase 1 — App skeleton and visual baseline

### Goal

Create a bootable FSDoctor application with the intended architecture and visual style, but without scan/check functionality yet.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 1.1 | Initialize Tauri 2 + Svelte project | App boots locally |
| 1.2 | Add `fsdoctor-core` crate | Core crate builds independently |
| 1.3 | Add frontend shell | Sidebar/top-level navigation |
| 1.4 | Add Catppuccin Mocha theme | Global CSS variables and base layout |
| 1.5 | Add JetBrains Mono font stack | Font fallback chain configured |
| 1.6 | Add placeholder views | Dashboard, Create Manifest, Check Backup, Reports, Settings |
| 1.7 | Add basic command roundtrip | Frontend can invoke a Rust command |

### Suggested frontend views

```text
Dashboard
Create Manifest
Check Backup
Report History
Report Detail
Settings
```

### Suggested base components

```text
AppShell
Card
Button
StatusBadge
ProgressPanel
ResultTable
FilePickerRow
EmptyState
```

### Minimal theme variables

```css
:root {
  --ctp-rosewater: #f5e0dc;
  --ctp-flamingo: #f2cdcd;
  --ctp-pink: #f5c2e7;
  --ctp-mauve: #cba6f7;
  --ctp-red: #f38ba8;
  --ctp-maroon: #eba0ac;
  --ctp-peach: #fab387;
  --ctp-yellow: #f9e2af;
  --ctp-green: #a6e3a1;
  --ctp-teal: #94e2d5;
  --ctp-sky: #89dceb;
  --ctp-sapphire: #74c7ec;
  --ctp-blue: #89b4fa;
  --ctp-lavender: #b4befe;
  --ctp-text: #cdd6f4;
  --ctp-subtext1: #bac2de;
  --ctp-subtext0: #a6adc8;
  --ctp-overlay2: #9399b2;
  --ctp-overlay1: #7f849c;
  --ctp-overlay0: #6c7086;
  --ctp-surface2: #585b70;
  --ctp-surface1: #45475a;
  --ctp-surface0: #313244;
  --ctp-base: #1e1e2e;
  --ctp-mantle: #181825;
  --ctp-crust: #11111b;
}

body {
  margin: 0;
  background: var(--ctp-base);
  color: var(--ctp-text);
  font-family: "JetBrainsMono Nerd Font", "JetBrains Mono", ui-monospace, monospace;
}
```

### Acceptance criteria

- [x] `npm run tauri dev` starts FSDoctor successfully.
- [x] `cargo test -p fsdoctor-core` works independently of Tauri.
- [x] UI has a minimal Catppuccin Mocha appearance.
- [ ] Top-level views are navigable. (No views yet, just a single page app)
- [x] A test Tauri command can be called from the frontend and returns data.
- [x] There is no integrity logic in the frontend.
- [x] There is no Tauri dependency in `fsdoctor-core`.

---

## Phase 2 — Project database foundation

### Goal

Implement creation and opening of FSDoctor project databases.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 2.1 | Add SQL migrations | Initial schema under `src-tauri/migrations` or core-owned migration path |
| 2.2 | Implement DB open/create API | `ProjectDb::create`, `ProjectDb::open` |
| 2.3 | Implement project metadata | Project name, root path, timestamps, format version |
| 2.4 | Add Tauri commands | `create_project`, `open_project` |
| 2.5 | Add UI workflow | Create/open project screens |
| 2.6 | Add warning for DB inside backup root | User-facing safety warning |

### Initial schema

```sql
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
    last_seen_scan_id INTEGER NOT NULL,
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
```

### Acceptance criteria

- [x] A new `.fsdoctor.sqlite` file can be created.
- [x] Existing `.fsdoctor.sqlite` files can be opened.
- [x] DB format version is stored and checked.
- [x] Project root path is stored.
- [ ] App warns when the DB is being saved inside the backup folder. (frontend very minial ATM)
- [x] Corrupt/non-FSDoctor SQLite files produce a friendly error.
- [x] Database tests cover migration, create, open, and version check.

---

## Phase 3 — Filesystem scanner and path model

### Goal

Implement reliable filesystem traversal and metadata collection without hashing file contents yet.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 3.1 | Define `RelativePath` type | Root-relative path abstraction |
| 3.2 | Implement root-safe path conversion | Absolute path -> relative manifest path |
| 3.3 | Implement filesystem walker | Emits directories, files, symlinks, other entries |
| 3.4 | Implement metadata extraction | Size, mtime, readonly, entry kind |
| 3.5 | Implement skip policy | Symlinks/reparse points not followed |
| 3.6 | Add scanner tests | Temp-directory based tests |

### Core model

```rust
pub struct ScanRoot {
    pub root_path: PathBuf,
}

pub struct FsEntry {
    pub relative_path: RelativePath,
    pub absolute_path: PathBuf,
    pub kind: FsEntryKind,
    pub size_bytes: Option<u64>,
    pub mtime_ns: Option<i128>,
    pub readonly: bool,
}

pub enum FsEntryKind {
    File,
    Directory,
    Symlink,
    Other,
}
```

### Scanner rules

- Do not follow symlinks by default.
- Do not follow Windows junctions/reparse points by default.
- Do not panic on unreadable paths.
- Do not fail the scan because one file cannot be accessed.
- Store paths relative to the selected root.
- Preserve enough error detail for diagnostics.
- Avoid string concatenation for filesystem paths; use `Path`, `PathBuf`, `OsStr`, and `OsString`.

### Acceptance criteria

- [x] Scanner emits regular files.
- [x] Scanner emits directories.
- [x] Scanner records symlinks/reparse points as skipped/special entries and does not follow them.
- [x] Scanner records unreadable entries as errors instead of aborting.
- [x] Scanner produces stable root-relative paths.
- [x] Scanner does not include files outside the selected root.
- [x] Unit/integration tests cover normal files, nested directories, symlinks, empty directories, and unreadable entries where platform-supported.

---

## Phase 4 — BLAKE3 hashing engine

### Goal

Implement streamed BLAKE3 hashing for regular files.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 4.1 | Add hash abstraction | `HashAlgorithm`, `FileDigest` |
| 4.2 | Implement streamed file hashing | Chunked reads into BLAKE3 hasher |
| 4.3 | Detect changed-during-hash files | Metadata before/after hashing |
| 4.4 | Add hash tests | Known fixtures and mutation tests |
| 4.5 | Add cancellation hook | Long file hashing can be cancelled between chunks |

### Hash model

```rust
pub struct FileDigest {
    pub algorithm: HashAlgorithm,
    pub bytes: [u8; 32],
}

pub enum HashAlgorithm {
    Blake3,
}
```

### Hashing flow

```text
read metadata before hashing
open file
stream chunks into BLAKE3 hasher
periodically check cancellation token
read metadata after hashing
if size or mtime changed:
  mark CHANGED_DURING_SCAN
else:
  store digest
```

### Acceptance criteria

- [x] Regular files can be hashed with BLAKE3.
- [x] Hashes are stored internally as 32-byte binary values.
- [x] Hash hex conversion exists only for display/export.
- [x] Known input bytes produce expected BLAKE3 digest.
- [x] File mutation during hashing is detected where testable.
- [x] Cancellation can interrupt hashing of a large file.
- [x] Hashing errors are structured and user-reportable.

---

## Phase 5 — Manifest generation end-to-end

### Goal

Generate a complete file-tree manifest and persist it into the project database.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 5.1 | Implement manifest-generation service | Core API: `generate_manifest` |
| 5.2 | Implement scan lifecycle | Create running scan, complete/fail/cancel scan |
| 5.3 | Persist manifest entries | Upsert by project + relative path |
| 5.4 | Add progress model | Core progress structs |
| 5.5 | Add Tauri job wrapper | `start_manifest_generation`, `cancel_job` |
| 5.6 | Add frontend progress panel | Live counters and current phase |
| 5.7 | Add manifest summary report | Files, dirs, bytes, unreadable, skipped |

### Recommended pipeline

```text
walker
  -> bounded entry queue
      -> hashing worker(s)
          -> result queue
              -> single DB writer
```

SQLite writes should be batched through a single writer task/thread to avoid write contention.

### Progress payload

```ts
type JobProgress = {
  jobId: string;
  kind: "manifest_generation" | "integrity_check";
  phase: "walking" | "hashing" | "writing" | "finishing";
  filesSeen: number;
  dirsSeen: number;
  bytesSeen: number;
  filesHashed: number;
  bytesHashed: number;
  unreadable: number;
  currentPath?: string;
};
```

### UI behavior

The manifest generation page should show:

```text
Current operation: Creating integrity record
Files found
Directories found
Data processed
Files checked
Unreadable files
Current path
Cancel button
```

Avoid exact percentage in the MVP unless a pre-scan is added. A one-pass scanner can show counters instead.

### Acceptance criteria

- [ ] User can create a project and generate a manifest for a selected folder.
- [ ] Manifest entries are persisted into SQLite.
- [ ] Regular files include BLAKE3 hashes.
- [ ] Directories are recorded without content hashes.
- [ ] Symlinks/reparse points are recorded/skipped according to policy.
- [ ] Unreadable files are recorded and included in the report.
- [ ] Scan progress is visible in the UI.
- [ ] Manifest generation can be cancelled.
- [ ] Cancelled scans are marked as cancelled in the DB.
- [ ] Failed scans are marked as failed with an error message.
- [ ] Running the manifest generation twice updates existing entries without duplicating paths.

---

## Phase 6 — Integrity check engine

### Goal

Compare the current filesystem tree against a stored manifest and classify differences precisely.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 6.1 | Define check result taxonomy | `CheckResultKind` |
| 6.2 | Implement manifest loading | Efficient lookup by relative path |
| 6.3 | Implement current tree scan | Reuse scanner and hasher |
| 6.4 | Implement result classification | OK/missing/new/mismatch/etc. |
| 6.5 | Persist check results | `check_results` rows by scan ID |
| 6.6 | Add Tauri command | `start_integrity_check` |
| 6.7 | Add check progress UI | Live check progress |

### Result taxonomy

```text
OK
MISSING
NEW
HASH_MISMATCH
SIZE_MISMATCH
TYPE_CHANGED
UNREADABLE
CHANGED_DURING_CHECK
SKIPPED
```

### Default comparison logic

```text
load manifest entries
walk current filesystem tree
for each current entry:
  if not in manifest:
    NEW
  else if type changed:
    TYPE_CHANGED
  else if regular file and size differs:
    SIZE_MISMATCH
  else if regular file and size matches:
    hash and compare
    if hash differs:
      HASH_MISMATCH
    else:
      OK
  else:
    OK or SKIPPED depending on entry kind

after walk:
  manifest entries not seen in current tree:
    MISSING
```

For the MVP, do not hash files whose size already differs. Add a later advanced option for deep checking.

### Acceptance criteria

- [ ] Unchanged files are reported as `OK`.
- [ ] Deleted files are reported as `MISSING`.
- [ ] Added files are reported as `NEW`.
- [ ] Modified same-size files are reported as `HASH_MISMATCH`.
- [ ] Modified different-size files are reported as `SIZE_MISMATCH`.
- [ ] File/directory replacements are reported as `TYPE_CHANGED`.
- [ ] Unreadable current files are reported as `UNREADABLE`.
- [ ] Changed-during-check files are reported as `CHANGED_DURING_CHECK`.
- [ ] Results are persisted in `check_results`.
- [ ] Integrity check can be cancelled.
- [ ] Check summary can be loaded after app restart.

---

## Phase 7 — Reports, history, filtering, and CSV export

### Goal

Make scan/check output useful for a non-technical user and exportable for offline records.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 7.1 | Implement summary queries | Counts by result/status |
| 7.2 | Implement report history | List past scans/checks |
| 7.3 | Implement report detail view | Filterable result table |
| 7.4 | Implement CSV export | Manifest and integrity CSVs |
| 7.5 | Add pagination | Large result sets do not freeze UI |
| 7.6 | Add friendly status mapping | Healthy / Needs attention / Incomplete |

### User-facing report categories

```text
Healthy:
  OK

Needs attention:
  HASH_MISMATCH
  SIZE_MISMATCH
  TYPE_CHANGED
  MISSING

Informational:
  NEW
  SKIPPED

Could not check:
  UNREADABLE
  CHANGED_DURING_CHECK
```

### Integrity summary model

```rust
pub struct IntegritySummary {
    pub scan_id: i64,
    pub total_files_checked: u64,
    pub total_dirs_checked: u64,
    pub total_bytes_read: u64,
    pub ok: u64,
    pub missing: u64,
    pub new: u64,
    pub hash_mismatch: u64,
    pub size_mismatch: u64,
    pub type_changed: u64,
    pub unreadable: u64,
    pub skipped: u64,
}
```

### CSV formats

Manifest-generation CSV:

```csv
relative_path,entry_kind,size_bytes,mtime_ns,status,hash_algorithm,hash_hex,error_message
```

Integrity-check CSV:

```csv
relative_path,result_kind,expected_kind,actual_kind,expected_size_bytes,actual_size_bytes,expected_hash_hex,actual_hash_hex,message
```

CSV export rule:

```text
CSV export reads from the database.
CSV export must not rescan or rehash the filesystem.
```

### Acceptance criteria

- [ ] Dashboard shows last manifest generation and last integrity check.
- [ ] Report history lists past scans with status, date, and summary.
- [ ] Integrity report detail page supports filtering by result kind.
- [ ] Large reports are paginated or virtualized.
- [ ] Manifest-generation CSV can be exported.
- [ ] Integrity-check CSV can be exported.
- [ ] Exported CSV includes enough information to identify each affected file.
- [ ] CSV export works after app restart.
- [ ] Healthy/incomplete/problem states are clear to non-technical users.

---

## Phase 8 — Windows hardening and large-tree performance

### Goal

Make FSDoctor robust enough for real external disks, old file shares, long paths, huge file counts, and non-technical users.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 8.1 | Long-path audit | Path handling reviewed on Windows |
| 8.2 | Reparse point handling | Junctions/symlinks not followed unexpectedly |
| 8.3 | External disk error handling | Drive disconnect and I/O errors handled |
| 8.4 | Performance pass | Large directory benchmarks |
| 8.5 | Progress throttling | UI remains responsive during large scans |
| 8.6 | DB batching tuning | Insert/update performance acceptable |
| 8.7 | Memory usage check | Millions of files do not require loading everything into RAM unnecessarily |

### Windows-specific concerns

- Long paths.
- Unicode path display.
- Junctions/reparse points.
- Locked files.
- Permission-denied files.
- Antivirus interference.
- External disk sleep/disconnect.
- Slow USB disks.
- Very large numbers of small files, such as `.git` directories.

### Performance principles

```text
Use streaming reads.
Use bounded queues.
Use a single DB writer.
Batch DB writes.
Throttle UI progress events.
Avoid unbounded in-memory result accumulation.
Avoid excessive parallelism on external HDDs.
```

### Acceptance criteria

- [ ] App remains responsive during a large scan.
- [ ] Progress updates do not overwhelm the frontend.
- [ ] DB writes are batched.
- [ ] Large trees with many small files are handled without excessive memory growth.
- [ ] Disconnecting/removing a drive results in a failed/incomplete report, not an app crash.
- [ ] Locked/unreadable files are reported individually.
- [ ] Symlinks and junctions do not cause infinite recursion.
- [ ] App gives useful messages for common Windows filesystem errors.

---

## Phase 9 — Packaging, distribution, and user documentation

### Goal

Produce a distributable Windows-friendly app with enough documentation for a non-technical user.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 9.1 | App metadata | Name, icon, version, author configured |
| 9.2 | Windows build | Installer/package generated |
| 9.3 | Smoke-test clean install | App works on a clean Windows machine/VM |
| 9.4 | User quickstart | `docs/user-quickstart.md` |
| 9.5 | Troubleshooting guide | `docs/troubleshooting.md` |
| 9.6 | Backup safety notes | Explains DB should be outside backup drive |

### User documentation topics

- What FSDoctor does.
- What FSDoctor does not do.
- How to create an integrity record.
- How to run a backup check.
- How to read the report.
- What “new”, “missing”, “changed”, and “could not check” mean.
- Why the FSDoctor database should be stored somewhere safe.
- What to do if files are reported as corrupted or unreadable.

### Acceptance criteria

- [ ] App can be installed and launched by a non-technical Windows user.
- [ ] App has a recognizable icon/name/version.
- [ ] First-use workflow is understandable without reading developer docs.
- [ ] User docs explain the normal workflow.
- [ ] Troubleshooting docs cover missing drive, unreadable files, cancelled scans, and corrupted project DB.
- [ ] Generated installer/package is reproducible from documented commands.

---

## Phase 10 — Post-MVP PAR2/parity support

### Goal

Add optional repair capability for selected directories without requiring a full duplicate backup copy.

### Milestones

| ID | Milestone | Deliverable |
|---:|-----------|-------------|
| 10.1 | Define parity DB schema | `parity_sets`, `parity_files` |
| 10.2 | Define parity backend trait | `ParityBackend` abstraction |
| 10.3 | Evaluate backend strategy | bundled sidecar vs user-provided executable vs library |
| 10.4 | Implement create parity workflow | Directory-scoped parity creation |
| 10.5 | Implement verify parity workflow | Verify available recovery data |
| 10.6 | Implement repair workflow | Repair files if enough parity exists |
| 10.7 | Add parity UI | Protection status and repair actions |

### Suggested parity abstraction

```rust
pub trait ParityBackend {
    fn create_set(&self, request: CreateParityRequest) -> Result<CreateParityReport>;
    fn verify_set(&self, request: VerifyParityRequest) -> Result<VerifyParityReport>;
    fn repair_set(&self, request: RepairParityRequest) -> Result<RepairParityReport>;
}
```

### Suggested parity tables

```sql
CREATE TABLE parity_sets (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    root_relative_path TEXT NOT NULL,
    par2_path TEXT NOT NULL,
    redundancy_percent INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    status TEXT NOT NULL,
    FOREIGN KEY(project_id) REFERENCES projects(id)
);

CREATE TABLE parity_files (
    id INTEGER PRIMARY KEY,
    parity_set_id INTEGER NOT NULL,
    relative_path TEXT NOT NULL,
    FOREIGN KEY(parity_set_id) REFERENCES parity_sets(id)
);
```

### PAR2 design rules

- Do not create one giant 2 TB parity set.
- Prefer directory-scoped parity sets.
- Keep parity optional.
- Warn the user that parity data consumes additional space.
- Integrate repair carefully and require clear confirmation before modifying files.

Example grouping:

```text
/photos/2009        -> photos_2009.par2
/documents/tax      -> documents_tax.par2
/projects/archive   -> projects_archive.par2
```

### Acceptance criteria

- [ ] Parity backend is isolated behind a trait/interface.
- [ ] App can create parity for a selected subdirectory.
- [ ] App can verify a parity set.
- [ ] App can identify whether repair is possible.
- [ ] App asks for explicit confirmation before repairing files.
- [ ] Repair results are reported clearly.
- [ ] Licensing/distribution implications of the chosen PAR2 backend are documented.

---

# 6. Cross-phase acceptance checklist

This checklist should hold true by the end of the MVP.

## Functional

- [ ] Create project DB.
- [ ] Open project DB.
- [ ] Select backup root.
- [ ] Generate manifest.
- [ ] Store per-file BLAKE3 hashes.
- [ ] Check backup against manifest.
- [ ] Detect missing files.
- [ ] Detect new files.
- [ ] Detect same-size content corruption.
- [ ] Detect size changes.
- [ ] Detect file/type replacements.
- [ ] Report unreadable files.
- [ ] Export CSV reports.
- [ ] Show report history.
- [ ] Cancel long-running jobs.

## UX

- [ ] UI is minimalistic.
- [ ] UI uses Catppuccin Mocha.
- [ ] UI uses JetBrains Mono Nerd Font fallback stack.
- [ ] Main workflows are wizard-like and easy to follow.
- [ ] Technical details are hidden by default.
- [ ] Reports use clear, non-technical categories.

## Reliability

- [ ] Unreadable files do not abort scans.
- [ ] Drive disconnect does not crash the app.
- [ ] Cancelled scans are marked as cancelled.
- [ ] Failed scans are marked as failed.
- [ ] Running scan state is not misreported as successful.
- [ ] Database schema version is checked.
- [ ] Project DB is not silently overwritten.

## Performance

- [ ] Large files are streamed, not loaded into memory.
- [ ] DB writes are batched.
- [ ] UI progress events are throttled.
- [ ] Large result sets are paginated/virtualized.
- [ ] Memory usage remains bounded for large trees.

## Testability

- [ ] Core logic can be tested without Tauri.
- [ ] Scanner tests use temp directories.
- [ ] Checker tests cover all result kinds.
- [ ] CSV export tests cover escaping and missing fields.
- [ ] Database migration tests run in CI.

---

# 7. Suggested Tauri command surface

Keep commands coarse-grained and workflow-oriented.

```rust
#[tauri::command]
async fn create_project(
    app: AppHandle,
    request: CreateProjectRequest,
) -> Result<ProjectDto, CommandError>;

#[tauri::command]
async fn open_project(
    app: AppHandle,
    db_path: PathBuf,
) -> Result<ProjectDto, CommandError>;

#[tauri::command]
async fn start_manifest_generation(
    app: AppHandle,
    request: StartManifestRequest,
) -> Result<JobStartedDto, CommandError>;

#[tauri::command]
async fn start_integrity_check(
    app: AppHandle,
    request: StartCheckRequest,
) -> Result<JobStartedDto, CommandError>;

#[tauri::command]
async fn cancel_job(
    app: AppHandle,
    job_id: String,
) -> Result<(), CommandError>;

#[tauri::command]
async fn get_scan_summary(
    app: AppHandle,
    scan_id: i64,
) -> Result<ScanSummaryDto, CommandError>;

#[tauri::command]
async fn list_scan_results(
    app: AppHandle,
    request: ListScanResultsRequest,
) -> Result<PagedResultsDto, CommandError>;

#[tauri::command]
async fn export_scan_csv(
    app: AppHandle,
    request: ExportCsvRequest,
) -> Result<ExportedFileDto, CommandError>;
```

Suggested app events:

```text
fsdoctor://job-progress
fsdoctor://job-completed
fsdoctor://job-failed
fsdoctor://job-cancelled
```

---

# 8. Suggested frontend structure

```text
src/lib/
├── api/
│   ├── commands.ts
│   └── events.ts
│
├── components/
│   ├── AppShell.svelte
│   ├── Card.svelte
│   ├── Button.svelte
│   ├── ProgressPanel.svelte
│   ├── StatusBadge.svelte
│   ├── ResultTable.svelte
│   ├── FilePickerRow.svelte
│   └── EmptyState.svelte
│
├── stores/
│   ├── project.ts
│   ├── jobs.ts
│   └── reports.ts
│
├── theme/
│   └── catppuccin.css
│
└── views/
    ├── Dashboard.svelte
    ├── CreateManifest.svelte
    ├── CheckBackup.svelte
    ├── ReportHistory.svelte
    ├── ReportDetail.svelte
    └── Settings.svelte
```

---

# 9. Test matrix

## Core unit tests

| Area | Tests |
|------|-------|
| Path handling | root-relative conversion, path outside root rejection, display conversion |
| Hashing | known digest, empty file, large file, cancellation |
| Scanner | nested dirs, empty dirs, regular files, symlinks, unreadable paths |
| Checker | OK, missing, new, size mismatch, hash mismatch, type changed, unreadable |
| Reports | summary counts, category mapping |
| CSV | escaping, empty values, binary hash -> hex |
| DB | migrations, inserts, upserts, version checks |

## Integration tests

Use temporary directories to simulate backup trees:

```text
create manifest for small tree
check unchanged tree -> all OK
delete file -> MISSING
add file -> NEW
modify file same size -> HASH_MISMATCH
modify file different size -> SIZE_MISMATCH
replace file with directory -> TYPE_CHANGED
make file unreadable -> UNREADABLE, where platform-supported
include symlink -> SKIPPED/recorded, not followed
cancel large scan -> CANCELLED
```

## Manual Windows tests

```text
external USB drive or simulated external path
large nested tree
many small files
long paths
locked file
read-only file
disconnected drive during scan
folder containing junction/symlink
CSV export opened in spreadsheet app
```

---

# 10. Recommended implementation order

The most practical order is:

```text
0. Product/technical foundation
1. App skeleton and visual baseline
2. Project database foundation
3. Filesystem scanner and path model
4. BLAKE3 hashing engine
5. Manifest generation end-to-end
6. Integrity check engine
7. Reports, history, filtering, and CSV export
8. Windows hardening and large-tree performance
9. Packaging, distribution, and user documentation
10. Post-MVP PAR2/parity support
```

Do not start with PAR2. Do not start with packaging. Do not over-optimize hashing before the scanner, DB, and report model are stable.

---

# 11. MVP exit criteria

FSDoctor can be considered MVP-complete when this demo works end-to-end:

1. Launch FSDoctor on Windows.
2. Create a new project database outside the backup folder.
3. Select a test backup folder.
4. Generate an integrity manifest.
5. Close and reopen FSDoctor.
6. Open the project database.
7. Run an integrity check.
8. See a healthy report.
9. Modify one file without changing its size.
10. Delete one file.
11. Add one new file.
12. Run the check again.
13. See:
    - one `HASH_MISMATCH`,
    - one `MISSING`,
    - one `NEW`,
    - remaining files `OK`.
14. Export the report to CSV.
15. Open the CSV and identify the affected paths.
16. Cancel a long-running scan and see it recorded as cancelled, not successful.
