# Specification

This document defines the Phase 0 product and technical specification for `FSDoctor`.
It is intentionally written before the scanner, database, and integrity-checking implementation so later development can be evaluated against stable requirements.

## Product summary

`FSDoctor` is a cross-platform desktop application for checking the integrity of mostly-static backup or file-share archives.

The first production target is a non-technical Windows user with a large backup stored on external disks.
The application must make integrity checking understandable without requiring command-line tools, filesystem expertise, or knowledge of cryptographic hashes.

The application creates an integrity record for a selected filesystem tree, stores that record in a SQLite project database, and later checks the same filesystem tree against the stored record.

## Core problem

A backup can become partially corrupted while still appearing browsable.
Operating-system disk tools can help detect storage-device problems, but they generally do not answer these project-specific questions:

- Which files were present when the backup was recorded?
- Which files can still be read today?
- Which files have changed since the integrity record was created?
- Which files are missing?
- Which files have matching size but different content?
- Which files could not be checked?

`FSDoctor` answers those questions by recording per-file content hashes and metadata, then comparing later filesystem state against that record.

## Target user

The primary user is not expected to be technically proficient.
The application must therefore provide:

- a graphical workflow;
- clear status summaries;
- minimal terminology;
- safe defaults;
- useful error explanations;
- CSV exports for hand-off to a more technical helper;
- no requirement to use shells, scripts, or external CLI tools.

Technical details may be available behind expandable sections, logs, CSV exports, or developer diagnostics, but they must not be necessary for normal operation.

## Supported platforms

The MVP is designed around these platform priorities:

1. Windows, especially Windows 10/11 with external disks.
2. Linux for development and validation.
3. macOS as a future compatibility target, unless packaging constraints make it expensive.

The core crate must avoid assuming a single platform unless the behavior is explicitly platform-specific and documented.

## Terminology

### Backup root

The directory selected by the user as the root of the backup tree.
All paths stored in the manifest are relative to this directory.

### FSDoctor project

A SQLite database file containing project metadata, scan history, manifest entries, and check results.
The project file is the authoritative integrity record.

### Integrity record

The stored manifest state produced by scanning a backup root.
It contains filesystem entry metadata and per-file content hashes.

### Manifest generation

A job that walks the selected backup root, records directories and special entries, hashes regular files, and stores the resulting manifest entries in the project database.

### Integrity check

A job that walks the current backup root and compares the current filesystem state against the stored manifest.

### Report

A persisted summary and detailed result set produced by a manifest generation or integrity check job.
Reports are stored in the project database and may be exported to CSV.

### Parity set

A future post-MVP repair mechanism, most likely implemented via a PAR2-compatible backend.
Parity sets are not part of the MVP.

## MVP scope

The MVP must provide the following capabilities:

- create a new FSDoctor project database;
- open an existing FSDoctor project database;
- select a backup root directory;
- generate a per-file BLAKE3 manifest for regular files;
- record directories and skipped special entries;
- store the manifest in SQLite;
- display manifest-generation progress;
- display manifest-generation summary reports;
- run an integrity check against a stored manifest;
- display integrity-check progress;
- display integrity-check summary reports;
- display detailed integrity-check results;
- export manifest-generation reports to CSV;
- export integrity-check reports to CSV;
- handle unreadable files without aborting the whole job;
- support cancelling long-running jobs;
- keep previous scan/check reports in the project database.

## Post-MVP scope

The following capabilities are expected later but must not block the MVP:

- PAR2 parity creation;
- PAR2 parity verification;
- file repair using available PAR2 recovery data;
- scheduled checks;
- resumable jobs after application restart;
- disk health checks such as SMART status;
- multiple backup roots in one project;
- tree-level Merkle summaries;
- advanced performance tuning profiles;
- deep analytics and trend charts;
- automatic cloud or external-device synchronization;
- automatic backup creation.

## Non-goals

`FSDoctor` is not a backup program.
It must not silently copy or modify user data during MVP workflows.

`FSDoctor` is not a replacement for redundant storage.
It can detect corruption, missing files, and unexpected changes, but the MVP cannot repair data.

`FSDoctor` is not a forensic tool.
It should not try to prove intent, identify malware, reconstruct deleted data, or interpret application-specific file formats.

`FSDoctor` is not a real-time filesystem monitor.
The MVP works through explicit user-triggered scan and check jobs.

`FSDoctor` is not a disk-health dashboard.
Disk-health integration may be added later, but Phase 0 and MVP focus on file-content integrity.

## Functional requirements

### Project management

The application shall allow the user to create a project database at a user-selected location.

The application shall warn when the project database is placed inside the backup root, because corruption of the backup storage may also corrupt the reference record.

The application shall allow the user to open an existing project database.

The application shall store the original backup root path, but it shall also allow the user to select a new current location when the original path is not available.

### Manifest generation

The application shall walk the selected backup root recursively.

The application shall record filesystem entries using root-relative paths.

The application shall hash regular files with BLAKE3.

The application shall store hashes as binary values in SQLite.

The application shall convert hashes to hexadecimal only for display, logs, and CSV export.

The application shall record unreadable entries as report entries rather than treating them as job-fatal errors.

The application shall detect files that changed while they were being hashed when possible by comparing metadata before and after hashing.

The application shall not follow symbolic links, junctions, or other reparse-point-like entries by default.

### Integrity checking

The application shall compare the current filesystem tree against the stored manifest.

The application shall report files that are present in the manifest but absent in the current tree as missing.

The application shall report files that are present in the current tree but absent in the manifest as new.

The application shall report regular files with mismatching size as size mismatches.

The application shall report regular files with matching size but different content hash as hash mismatches.

The application shall report entries whose filesystem type changed as type changes.

The application shall report unreadable current entries without aborting the whole check.

The application shall persist integrity-check results in the project database.

### Reporting

The application shall provide a high-level summary after manifest generation.

The application shall provide a high-level summary after integrity checks.

The application shall provide a detailed table for integrity-check results.

The application shall allow filtering results by severity or result kind.

The application shall export manifest-generation reports to CSV.

The application shall export integrity-check reports to CSV.

CSV export shall be derived from persisted database records and must not rescan the filesystem.

### Job control

The application shall represent long-running operations as jobs.

The application shall provide progress updates during long-running jobs.

The application shall allow cancellation of manifest-generation and integrity-check jobs.

Cancellation shall leave a persisted scan record marked as cancelled.

## Non-functional requirements

### Usability

The UI must prioritize simple status wording over technical detail.

Examples of preferred wording:

- `Backup appears healthy.`
- `No corrupted files were found.`
- `Some files could not be checked.`
- `Some files are missing.`

Examples of wording to avoid in primary UI:

- `All BLAKE3 digests matched manifest rows.`
- `The manifest relation has zero hash divergence.`

### Reliability

A single unreadable file must not abort the whole scan or check.

A disconnected drive, inaccessible root, database error, or unrecoverable internal error may fail a job.

The application must preserve enough context to explain what happened.

### Safety

MVP workflows must be read-only with respect to the backup data.

The application may create or update its project database and exported reports.

The application must not delete, overwrite, repair, or rewrite backup files in MVP workflows.

### Performance

The application must handle very large trees containing many small files.

The design must avoid loading all file contents into memory.

The implementation should stream file contents through the hasher.

Database writes should be batched.

UI progress updates should be throttled.

### Maintainability

The core functionality must live in `fsdoctor-core`.

The `fsdoctor-core` crate must not depend on Tauri.

The Tauri application crate must be a thin shell around commands, job orchestration, dialogs, and event emission.

The frontend must not duplicate backend integrity logic.

## Phase 0 deliverables

Phase 0 is complete when the repository contains documentation that defines:

- the product specification;
- the MVP scope;
- post-MVP scope;
- non-goals;
- architecture;
- database model;
- scan and check pipeline;
- UI and UX guidelines;
- quality gates;
- documentation generation workflow.

## Phase 0 acceptance criteria

Phase 0 is accepted when:

- the mdBook builds successfully;
- the development documentation is linked from the book;
- the generated backend Rust docs are linked from the book output;
- the generated frontend TypeScript docs are linked from the book output;
- the generated Storybook UI docs are linked from the book output;
- the MVP scope is explicit;
- post-MVP work is explicitly excluded from MVP acceptance;
- Phase 1 can start with database migrations and core domain types without reopening product-scope questions.

## Phase 1 handoff

Phase 1 should begin with implementation of:

- core domain enums and value types;
- SQLite migrations;
- project database create/open logic;
- scan record persistence;
- basic test fixtures for temporary filesystem trees.
