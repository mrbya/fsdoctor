# Contributing

This document defines the contribution workflow for `FSDoctor`.

The project uses strict checks and intentionally keeps the integrity logic separated from the desktop shell and frontend UI.

## Development principles

Prefer boring, explicit, testable code.

Important principles:

- keep `fsdoctor-core` independent from Tauri;
- keep long-running operations job-based;
- keep filesystem errors structured;
- treat unreadable files as report entries where possible;
- avoid panic-based control flow;
- avoid duplicating backend logic in the frontend;
- write tests for behavior, not only implementation details;
- update docs when behavior changes.

## Repository areas

```text
src/                            # Frontend Svelte/TypeScript code
src-tauri/                      # Tauri application crate
src-tauri/crates/fsdoctor-core/ # Core Rust logic
docs/book/                      # mdBook source
docs/dev/                       # generated-doc landing/readme inputs
```

## Before starting work

Read these pages first:

- [Specification](./specification.md)
- [Architecture](./architecture.md)
- [Data model](./data-model.md)
- [Scan and check pipeline](./scan-and-check-pipeline.md)
- [Quality gates](./quality-gates.md)

## Normal development loop

Run the app during development:

```bash
just dev
```

Run focused checks while iterating:

```bash
just check
just test
```

Format code:

```bash
just fmt
```

Before committing:

```bash
just pre-commit
```

## Adding backend functionality

Backend business logic belongs in `fsdoctor-core` unless it is specifically about desktop integration.

Examples that belong in `fsdoctor-core`:

- manifest-generation logic;
- integrity-check classification;
- SQLite persistence;
- CSV export;
- path normalization;
- hash computation.

Examples that belong in the Tauri crate:

- file dialogs;
- command registration;
- job registry wiring;
- event emission to frontend;
- app-level state.

## Adding frontend functionality

Frontend code should communicate with the backend through typed command wrappers and event subscriptions.

Frontend code should not:

- hash files;
- classify integrity results independently;
- write to the project database;
- assume OS-specific path behavior beyond display needs.

Reusable components should receive typed data and should be covered by Storybook when practical.

## Adding database changes

Database changes should be made through migrations.

When changing the schema:

- update the migration;
- update data model docs;
- add or update database tests;
- update query preparation/offline metadata if required;
- consider migration compatibility with existing project files.

## Adding result kinds or statuses

When adding a new status/result kind:

- update Rust enum definitions;
- update SQLite serialization/deserialization;
- update data model docs;
- update scan/check pipeline docs;
- update frontend severity mapping;
- update CSV export;
- update tests.

## Adding dependencies

Before adding a dependency, verify that it is necessary and compatible with project goals.

Consider:

- maintenance status;
- license;
- binary size impact;
- audit surface;
- whether the dependency will be used in production or only tests;
- whether it will pass unused-dependency checks.

## Documentation expectations

Documentation is part of the implementation.

Update the book when changing:

- user workflows;
- architecture;
- database schema;
- scan/check behavior;
- UI wording;
- quality gates;
- docs generation.

Add Rustdoc/TypeDoc comments for new APIs.

Add Storybook stories for reusable visual states.

## Pull request checklist

Before opening or merging a change, verify:

- code is formatted;
- Rust and frontend checks pass;
- tests pass;
- unused dependency checks pass or exceptions are justified;
- audits pass or findings are triaged;
- documentation was updated if behavior changed;
- user-facing wording remains non-technical;
- core logic remains outside the Tauri shell.

## Contribution acceptance criteria

A contribution is acceptable when:

- it fits the documented architecture;
- it passes the quality gates relevant to the change;
- it includes tests for meaningful behavior;
- it does not introduce unnecessary dependencies;
- it does not weaken user-facing clarity;
- it updates documentation when needed.
