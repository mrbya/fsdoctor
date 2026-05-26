# UI and UX guidelines

This document defines the user-interface principles for `FSDoctor`.

The application is intended for users who should not need to understand filesystems, hash algorithms, SQLite, or command-line tools.

## Visual direction

`FSDoctor` should use a minimal dark interface.

Baseline style:

- Catppuccin Mocha color palette;
- JetBrains Mono Nerd Font where available;
- simple cards;
- clear status badges;
- sparse navigation;
- no dense dashboards unless they answer an immediate question.

The UI should feel calm and diagnostic rather than noisy.

## Primary UX rule

The primary UI must explain backup health in plain language.

Good:

```text
Backup appears healthy.
No corrupted files were found.
3 files could not be checked.
22 new files were found.
```

Avoid in primary UI:

```text
All BLAKE3 digests matched the manifest relation.
The check_results table contains zero divergent rows.
```

Technical detail is useful, but it belongs in expandable details, CSV exports, logs, and developer documentation.

## Main workflows

The MVP has three primary workflows.

### Create integrity record

Used when the user wants to record the current known-good state of a backup.

Steps:

1. select backup folder;
2. select where to save the FSDoctor project database;
3. review warning if database is inside the backup folder;
4. start scan;
5. watch progress;
6. view summary;
7. optionally export CSV.

Suggested CTA:

```text
Create integrity record
```

### Run backup check

Used when the user wants to verify a backup against an existing integrity record.

Steps:

1. open FSDoctor project database;
2. confirm or reselect backup folder;
3. start check;
4. watch progress;
5. view summary;
6. inspect details if needed;
7. optionally export CSV.

Suggested CTA:

```text
Run backup check
```

### View reports

Used when the user wants to inspect previous scans/checks.

Features:

- list report history;
- open report summary;
- filter detailed results;
- export CSV.

Suggested CTA:

```text
View reports
```

## Navigation

Recommended navigation:

```text
Dashboard
Create integrity record
Run backup check
Reports
Settings
```

The navigation should be stable and shallow.
Avoid burying the two main workflows under multiple menu levels.

## Dashboard

When no project is open, the dashboard should show:

- create new integrity record;
- open existing FSDoctor project;
- brief explanation of what FSDoctor does.

When a project is open, the dashboard should show:

- project name;
- backup folder path;
- last manifest generation status;
- last integrity check status;
- quick action to run a check;
- warning if the backup root is unavailable.

## Status language

Recommended top-level health statuses:

### Healthy

Use when an integrity check completed and found no attention-requiring entries.

Suggested text:

```text
Backup appears healthy.
No corrupted or missing files were found.
```

### Attention required

Use when missing, hash-mismatched, size-mismatched, or type-changed entries exist.

Suggested text:

```text
Some files need attention.
Review the details below.
```

### Check incomplete

Use when the check completed but some entries could not be checked.

Suggested text:

```text
The check completed, but some files could not be checked.
```

### Failed

Use when the job itself failed.

Suggested text:

```text
The check could not be completed.
```

### Cancelled

Use when the user cancelled the job.

Suggested text:

```text
The check was cancelled.
Partial results may be available.
```

## Severity mapping

Detailed result kinds map to UI groups:

| UI group | Result kinds | Tone |
| --- | --- | --- |
| Healthy | `ok` | calm/success |
| Needs attention | `missing`, `hash_mismatch`, `size_mismatch`, `type_changed` | warning/error |
| Could not check | `unreadable`, `changed_during_check` | warning/incomplete |
| Informational | `new`, `skipped` | neutral/info |

The UI should not overstate informational results.
A new file may be expected if the backup was intentionally updated.
A skipped symlink may be expected policy.

## Report summary cards

Manifest-generation summary should show:

- directories found;
- files found;
- files hashed;
- total data read;
- skipped entries;
- unreadable entries;
- changed-during-scan entries;
- duration.

Integrity-check summary should show:

- files checked;
- directories checked;
- total data read;
- healthy entries;
- files needing attention;
- missing files;
- unreadable entries;
- new files;
- duration.

## Detailed result table

Recommended columns:

```text
Status
Path
Expected
Actual
Message
```

The table should support filters:

```text
All
Needs attention
Could not check
New files
Missing files
Healthy
```

For very large result sets, the backend should provide pagination.
The frontend should avoid rendering huge tables all at once.

## Progress display

Progress should focus on live counters, not exact percentages.

Recommended progress panel fields:

- current phase;
- files seen;
- files hashed or checked;
- data read;
- unreadable entries;
- elapsed time;
- current path, optionally truncated.

Possible phases:

```text
Preparing
Scanning folders
Hashing files
Checking files
Writing report
Finishing
Cancelling
```

## Dialog and warning text

### Database inside backup warning

```text
The integrity database is being saved inside the backup folder.
For better protection, save it somewhere else, such as your main computer or another drive.
```

### Missing backup root

```text
The original backup folder was not found.
Select the current location of this backup to continue.
```

### Unreadable files

```text
Some files could not be read.
They may be locked, inaccessible, or affected by a drive problem.
```

### Hash mismatch

```text
The file content does not match the integrity record.
This may indicate corruption or an intentional change after the record was created.
```

### Missing files

```text
These files were present when the integrity record was created, but they are missing now.
```

## Technical detail disclosure

Use progressive disclosure.

Primary view:

```text
Some files need attention.
```

Details view:

```text
Status: Hash mismatch
Path: photos/2012/img_001.jpg
Expected size: 4.2 MB
Actual size: 4.2 MB
```

Technical details:

```text
Expected BLAKE3: ...
Actual BLAKE3: ...
OS error: ...
```

## Theme baseline

Suggested CSS variables should mirror Catppuccin Mocha names.

Minimum semantic mapping:

```text
background: base/mantle/crust
surface: surface0/surface1/surface2
text: text/subtext1/subtext0
success: green
warning: yellow/peach
error: red
info: blue/sky
accent: mauve/lavender
```

## Font behavior

Preferred font stack:

```css
font-family: "JetBrainsMono Nerd Font", "JetBrains Mono", ui-monospace, monospace;
```

The UI must remain usable when JetBrains Mono Nerd Font is not installed.

## Accessibility

The UI should not rely on color alone.

Status badges should include text labels.

Tables should have readable spacing and persistent headers where practical.

Long paths should be selectable/copyable.

Progress and errors should be visible to screen readers where feasible.

## UI acceptance criteria

The UI/UX specification is accepted when:

- primary workflows are defined;
- status wording is non-technical;
- severity mapping is defined;
- report summary and detail structure are defined;
- Catppuccin Mocha and JetBrains Mono Nerd Font are established as baseline style choices;
- technical details are available but not required for normal operation;
- warnings for unsafe database placement and missing roots are specified.
