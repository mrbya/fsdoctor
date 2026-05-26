# Architecture

This document defines the high-level architecture of `FSDoctor`.

The main architectural goal is to keep the integrity-checking logic independent from the desktop shell and frontend framework.
The Rust core should be testable without launching Tauri or rendering the UI.

## System overview

`FSDoctor` consists of three main layers:

```text
Svelte frontend
  └── Tauri application shell
        └── fsdoctor-core
```

Each layer has a deliberately narrow responsibility.

## Layer responsibilities

### Svelte frontend

The frontend owns presentation and user interaction.

Responsibilities:

- route/view composition;
- minimal Catppuccin Mocha UI;
- user workflow screens;
- progress display;
- summary cards;
- detailed report tables;
- CSV export actions;
- frontend-side validation of form completeness;
- subscribing to Tauri events;
- invoking coarse-grained Tauri commands.

The frontend must not:

- walk the filesystem directly;
- hash file contents;
- interpret manifest correctness;
- write SQLite records directly;
- contain duplicate result-classification logic.

### Tauri application shell

The Tauri shell owns desktop integration and process-level orchestration.

Responsibilities:

- registering Tauri commands;
- opening file and directory dialogs;
- holding application state;
- managing long-running job handles;
- forwarding cancellation requests;
- converting core events to frontend events;
- converting core errors to UI-safe error payloads;
- integrating platform-specific desktop capabilities.

The Tauri shell should remain thin.
It should delegate durable logic to `fsdoctor-core`.

### `fsdoctor-core`

The core crate owns all project-specific integrity logic.

Responsibilities:

- domain model;
- project database access;
- SQLite migrations interface;
- filesystem walking;
- path normalization;
- file metadata collection;
- file hashing;
- manifest generation;
- integrity checking;
- report generation;
- CSV export;
- future parity backend abstraction.

The core crate must not depend on Tauri.

## Dependency direction

Allowed dependency direction:

```text
frontend -> Tauri commands/events -> fsdoctor-core
```

Rust crate dependency direction:

```text
fsdoctor application crate -> fsdoctor-core
```

Forbidden dependency direction:

```text
fsdoctor-core -> tauri
fsdoctor-core -> frontend assets
fsdoctor-core -> platform UI APIs
```

## Suggested Rust module layout

```text
src-tauri/crates/fsdoctor-core/src/
├── lib.rs
├── db/
│   ├── mod.rs
│   ├── connection.rs
│   ├── migrations.rs
│   ├── project.rs
│   ├── scans.rs
│   ├── manifest.rs
│   └── reports.rs
├── domain/
│   ├── mod.rs
│   ├── paths.rs
│   ├── digest.rs
│   ├── entries.rs
│   ├── scans.rs
│   └── results.rs
├── fs/
│   ├── mod.rs
│   ├── walker.rs
│   ├── metadata.rs
│   └── platform.rs
├── hashing/
│   ├── mod.rs
│   └── blake3.rs
├── manifest/
│   ├── mod.rs
│   └── generation.rs
├── check/
│   ├── mod.rs
│   └── engine.rs
├── report/
│   ├── mod.rs
│   ├── summary.rs
│   └── details.rs
├── csv/
│   ├── mod.rs
│   └── export.rs
├── jobs/
│   ├── mod.rs
│   ├── progress.rs
│   └── cancellation.rs
└── parity/
    ├── mod.rs
    └── backend.rs
```

The exact module names may change during implementation, but the boundaries should remain stable.

## Suggested Tauri module layout

```text
src-tauri/src/
├── lib.rs
├── main.rs
├── commands.rs
├── error.rs
├── events.rs
├── jobs.rs
└── state.rs
```

### `commands.rs`

Exposes coarse-grained Tauri commands such as:

- create project;
- open project;
- start manifest generation;
- start integrity check;
- cancel job;
- fetch scan summary;
- list scan results;
- export CSV.

### `events.rs`

Maps core progress/completion events into frontend-facing event payloads.

### `jobs.rs`

Maintains in-memory running job state.
A job registry is runtime state, not persistent business data.

### `state.rs`

Stores shared Tauri application state, such as open project references and the job registry.

## Frontend module layout

```text
src/lib/
├── api/
│   ├── commands.ts
│   └── events.ts
├── components/
│   ├── AppShell.svelte
│   ├── Button.svelte
│   ├── Card.svelte
│   ├── FilePickerRow.svelte
│   ├── ProgressPanel.svelte
│   ├── ResultTable.svelte
│   └── StatusBadge.svelte
├── stores/
│   ├── project.ts
│   ├── jobs.ts
│   └── reports.ts
├── theme/
│   └── catppuccin.css
└── views/
    ├── Dashboard.svelte
    ├── CreateManifest.svelte
    ├── CheckBackup.svelte
    ├── ReportDetail.svelte
    ├── ReportHistory.svelte
    └── Settings.svelte
```

## Command boundary

Tauri commands should be coarse-grained and workflow-oriented.

Good command examples:

```rust
start_manifest_generation(request)
start_integrity_check(request)
export_scan_csv(request)
```

Poor command examples:

```rust
hash_one_file(path)
insert_manifest_row(row)
classify_check_result(expected, actual)
```

The frontend should not coordinate low-level integrity operations.

## Event boundary

Long-running operations should publish progress events.

Event categories:

- job started;
- job progress;
- job completed;
- job cancelled;
- job failed.

Progress events should be throttled before crossing the Tauri/frontend boundary.
A filesystem tree with millions of files must not generate millions of UI events.

## Job model

Manifest generation, integrity checking, and CSV export are jobs.

A job has:

- an ID;
- a kind;
- a status;
- progress counters;
- optional cancellation token;
- optional persisted scan ID;
- optional error information.

Runtime job state is held by the Tauri shell.
Durable job results are held in SQLite.

## Persistence model

SQLite is the durable project state.

The database stores:

- project metadata;
- scan history;
- latest manifest entries;
- check results;
- report summaries;
- future parity metadata.

CSV files are exported artifacts only.
They are not the source of truth.

## Filesystem identity

Manifest entries are identified by root-relative paths.

The selected backup root may move between machines or drive letters.
The application should preserve the original root path as metadata but allow the user to choose a new current root when checking.

## Platform-specific behavior

Platform-specific filesystem behavior must be isolated behind small modules or functions.

Examples:

- Windows reparse-point detection;
- symlink classification;
- timestamp precision;
- readonly flag interpretation;
- long-path handling;
- non-UTF-8 path representation on Unix.

The first MVP may choose pragmatic limitations, but those limitations must be documented and surfaced as report statuses when relevant.

## Error handling

Core functions should return structured errors.

The Tauri layer should translate structured errors into frontend-safe messages.

The frontend should display a short user-facing message and optionally expose technical details.

Error handling principles:

- unreadable files are data/report events;
- disconnected root is a job failure;
- database write failure is a job failure;
- unsupported path representation is a report event when localized to one path;
- unsupported project database format is an open-project failure.

## Testability requirements

The core crate must support tests that create temporary filesystem trees and run manifest/check logic without Tauri.

Test categories:

- pure unit tests for domain model;
- database tests for migrations and queries;
- integration tests for scanning/checking temporary trees;
- CSV export tests;
- platform-specific filesystem tests where practical.

## Architecture acceptance criteria

The architecture is accepted when:

- `fsdoctor-core` contains no Tauri dependency;
- Tauri commands are coarse-grained;
- the frontend communicates through typed command/event wrappers;
- scan/check correctness can be tested in Rust without starting the GUI;
- SQLite is the durable source of truth;
- CSV exports are derived from persisted reports;
- platform-specific behavior is isolated and documented.
