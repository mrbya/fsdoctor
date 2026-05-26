# Scan and check pipeline

This document defines the manifest-generation and integrity-checking behavior for `FSDoctor`.

The pipeline must be reliable for large, mostly-static backup trees and friendly to non-technical users.

## Pipeline goals

The scan/check pipeline must:

- read file contents sequentially without loading whole files into memory;
- hash regular files with BLAKE3;
- record directories and skipped entries;
- avoid following symlinks and Windows junctions by default;
- preserve root-relative path identity;
- keep going when individual entries are unreadable;
- emit progress events;
- support cancellation;
- persist reports in SQLite.

## Manifest generation

Manifest generation creates or refreshes the integrity record for a backup root.

High-level flow:

```text
create scan row
walk backup root
classify each filesystem entry
record directories and skipped entries
hash regular files
persist manifest entries
update scan counters
mark scan completed, cancelled, or failed
```

## Integrity check

Integrity checking compares the current filesystem tree against the stored manifest.

High-level flow:

```text
create scan row
load manifest index
walk current backup root
classify each filesystem entry
compare current entries against expected entries
hash regular files when required
record check results
identify manifest entries not seen during walk
mark scan completed, cancelled, or failed
```

## Filesystem traversal

The walker recursively traverses the selected backup root.

Traversal rules:

- use platform-native path APIs;
- store root-relative paths in the database;
- do not construct filesystem paths by string concatenation;
- do not follow symlinks by default;
- do not follow Windows junctions/reparse points by default;
- treat traversal errors as entry-level results where possible;
- fail the job only when the root itself is unavailable or traversal cannot continue meaningfully.

## Entry classification

Each discovered entry is classified as:

```text
file
directory
symlink
other
```

MVP behavior:

| Entry kind | Manifest behavior | Check behavior |
| --- | --- | --- |
| `file` | Hash content and store metadata. | Compare type, size, and hash. |
| `directory` | Record directory metadata. | Verify that a directory exists. |
| `symlink` | Record as skipped by default. | Verify according to skipped-entry policy. |
| `other` | Record as skipped or unsupported. | Verify according to skipped-entry policy. |

## Symlinks and reparse points

Default policy:

```text
Do not follow symlinks, junctions, or reparse points.
```

Rationale:

- avoids recursive loops;
- avoids accidentally scanning data outside the selected backup root;
- avoids surprising users;
- keeps MVP behavior simple and safe.

The app should report these entries as skipped/recorded rather than failing the job.

A future advanced setting may allow following links, but it must include loop detection and root-boundary controls.

## Hashing regular files

Regular files are hashed using BLAKE3.

Recommended process:

```text
read metadata before hashing
open file
stream file contents through BLAKE3 hasher
read metadata after hashing
compare before/after size and modification timestamp
store hash if stable
mark changed_during_scan/check if unstable
```

The implementation must not read entire large files into memory.

Recommended implementation shape:

```text
open file
allocate reusable read buffer
loop:
  read chunk
  update hasher
finish digest
```

## Detecting files changed during processing

Even though backups should be mostly static, files can still change while scanning.
The application should detect common cases by comparing metadata before and after hashing.

For manifest generation:

```text
if size or modification time changed while hashing:
  store status changed_during_scan
  do not present the resulting hash as fully trustworthy
```

For integrity checking:

```text
if size or modification time changed while hashing:
  record changed_during_check
```

The UI should explain this as:

```text
The file changed while FSDoctor was checking it. Run the check again after the backup is idle.
```

## Manifest-generation statuses

Manifest generation may produce these entry statuses:

```text
hashed
recorded
skipped
unreadable
changed_during_scan
```

### `hashed`

A regular file was read successfully and its BLAKE3 digest was stored.

### `recorded`

A non-file entry, usually a directory, was recorded without content hashing.

### `skipped`

An entry was intentionally not processed, usually due to policy.

Examples:

- symlink;
- Windows junction;
- unsupported special file.

### `unreadable`

The entry could not be read or inspected.

Examples:

- permission denied;
- sharing violation;
- broken path;
- I/O error.

### `changed_during_scan`

A file appeared to change while it was being hashed.

## Integrity-check result kinds

Integrity checks may produce these result kinds:

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

### `ok`

The current entry matches the manifest expectation.

For regular files, this means:

- expected type is file;
- actual type is file;
- expected size equals actual size;
- expected hash equals actual hash.

### `missing`

The entry exists in the manifest but not in the current filesystem tree.

### `new`

The entry exists in the current filesystem tree but not in the manifest.

### `size_mismatch`

A regular file exists at the expected path, but its size differs.

Default behavior:

```text
Report size_mismatch and skip content hashing for that file.
```

A future deep-check mode may hash size-mismatched files as well.

### `hash_mismatch`

A regular file exists at the expected path, its size matches, but its content hash differs.

This is the closest MVP result to “corrupted file”.

### `type_changed`

The entry exists at the expected path, but its kind changed.

Examples:

- expected file, actual directory;
- expected directory, actual file;
- expected file, actual symlink.

### `unreadable`

The current entry could not be inspected or read.

This is a check-incomplete condition, not proof of corruption.

### `changed_during_check`

The file appeared to change while being checked.

This is a check-incomplete condition.

### `skipped`

The entry was skipped by policy.

## Comparison algorithm

Recommended high-level algorithm:

```text
load manifest entries into a lookup structure keyed by relative_path
create an empty seen set
walk current filesystem tree
for each current entry:
  compute relative_path
  mark relative_path seen
  if relative_path not in manifest:
    record new
    continue
  compare expected kind and actual kind
  if kind differs:
    record type_changed
    continue
  if directory:
    record ok
    continue
  if skipped/special:
    apply skipped-entry policy
    continue
  if file:
    compare size
    if size differs:
      record size_mismatch
      continue
    hash current file
    compare hash
    record ok or hash_mismatch

for each manifest entry not in seen set:
  record missing
```

## Performance model

The pipeline should be designed as bounded work queues.

Recommended architecture:

```text
walker
  -> bounded entry queue
      -> hashing worker(s)
          -> bounded result queue
              -> single database writer
```

Rationale:

- the walker should not outrun memory;
- hashing should be parallelizable but bounded;
- SQLite writes should be batched;
- a single writer avoids unnecessary write contention;
- external disks may become slower with too much parallelism.

The MVP may begin with a simpler implementation if it preserves the public job/progress model.

## Progress reporting

Progress events should contain counters, not exact percentages.

Exact percentages require a pre-scan to count all files and bytes, which adds I/O overhead.
The MVP should prefer one-pass scanning with live counters.

Useful counters:

- directories seen;
- files seen;
- bytes discovered;
- files hashed;
- bytes hashed;
- unreadable entries;
- skipped entries;
- current path, when useful;
- elapsed time.

Progress events should be throttled.
The UI does not need one event per file.

## Cancellation

Cancellation should be cooperative.

Cancellation checks should occur:

- before processing the next directory entry;
- before opening the next file;
- between large file read chunks;
- before committing large database batches;
- before emitting completion events.

When cancelled:

- the active scan row should be marked `cancelled`;
- partial detail records may remain for diagnostic purposes;
- the UI should clearly indicate that the job did not complete.

## Job failures

A job may fail when:

- the selected root cannot be opened;
- the project database cannot be read or written;
- a required migration fails;
- an internal invariant is violated;
- the drive disconnects in a way that prevents meaningful continuation.

Individual unreadable files are not job failures.
They are per-entry results.

## CSV export behavior

CSV export must read from persisted database records.
It must not rescan the filesystem.

Manifest report export should include:

```text
relative_path
entry_kind
size_bytes
mtime_ns
status
hash_algorithm
hash_hex
error_message
```

Integrity report export should include:

```text
relative_path
result_kind
expected_kind
actual_kind
expected_size_bytes
actual_size_bytes
expected_hash_hex
actual_hash_hex
message
```

## User-facing severity mapping

Detailed result kinds should map to simple UI groups:

| Group | Result kinds |
| --- | --- |
| Healthy | `ok` |
| Needs attention | `missing`, `hash_mismatch`, `size_mismatch`, `type_changed` |
| Could not check | `unreadable`, `changed_during_check` |
| Informational | `new`, `skipped` |

The UI should avoid calling every mismatch “corruption”.
Only `hash_mismatch` is strong evidence that file content differs despite matching size.

## Pipeline acceptance criteria

The pipeline specification is accepted when:

- manifest generation records directories, files, skipped entries, and unreadable entries;
- regular files are hashed with BLAKE3;
- integrity checks classify all expected MVP result kinds;
- unreadable individual files do not abort the job;
- symlinks and reparse points are not followed by default;
- progress reporting is defined;
- cancellation behavior is defined;
- CSV export is based on persisted results.
