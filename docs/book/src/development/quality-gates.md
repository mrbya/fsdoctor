# Quality gates

This document defines the expected quality gates for `FSDoctor` development.

The repository is intentionally configured with strict checks for both backend and frontend code.

## Quality goals

`FSDoctor` should be:

- reliable for large filesystem trees;
- explicit in error handling;
- conservative with dependencies;
- well documented;
- testable without the GUI;
- auditable for known vulnerabilities;
- free from unused dependencies;
- suitable for a non-technical user.

## Rust standards

Rust code should follow the existing strict lint philosophy.

Expected style:

- no `unwrap` in normal implementation code;
- no `expect` in normal implementation code unless the invariant is documented and unavoidable;
- no panic-based control flow;
- no unchecked indexing/slicing;
- no unchecked arithmetic when overflow is plausible;
- explicit conversions;
- documented public and private items;
- structured errors;
- tests for domain behavior;
- integration tests for filesystem/database behavior.

Test code may use more direct assertions, but should still avoid hiding important failure modes.

## Frontend standards

Frontend code should follow the existing TypeScript/Svelte checks.

Expected style:

- typed command wrappers;
- typed event payloads;
- no duplicated backend integrity logic;
- no unbounded rendering of huge result sets;
- accessible status labels;
- stable component boundaries;
- Storybook coverage for reusable UI components.

## Dependency standards

Dependencies should be added intentionally.

Before adding a dependency, consider:

- Is it needed for MVP?
- Is it maintained?
- Is it compatible with the project license?
- Does it substantially reduce implementation risk?
- Does it complicate packaging?
- Will it trigger audit or unused-dependency checks?

Unused dependency checks are part of the expected workflow.

## Security and vulnerability checks

Both backend and frontend dependency trees should be audited.

A vulnerability finding should be triaged as:

- not applicable;
- acceptable temporarily with documented rationale;
- requires dependency update;
- requires dependency removal/replacement;
- blocks release.

## Testing strategy

### Rust unit tests

Cover pure logic:

- path normalization;
- enum serialization;
- digest formatting;
- result classification;
- summary counting;
- CSV escaping.

### Rust integration tests

Use temporary filesystem trees to cover:

- unchanged tree produces `ok` results;
- missing file produces `missing`;
- new file produces `new`;
- same-size content modification produces `hash_mismatch`;
- changed file size produces `size_mismatch`;
- file replaced by directory produces `type_changed`;
- unreadable entry is recorded without aborting where the platform allows testing;
- symlink/reparse-point behavior is policy-compliant.

### Database tests

Cover:

- migrations apply cleanly;
- project metadata round-trip;
- scan lifecycle;
- manifest upsert;
- check-result insertion;
- report summary queries;
- pagination/filtering queries.

### Frontend tests

Cover:

- command wrapper behavior;
- report status mapping;
- component rendering for major statuses;
- table filtering;
- progress display state.

### Storybook

Reusable UI components should have stories for important states:

- idle;
- running;
- completed healthy;
- attention required;
- incomplete;
- failed;
- cancelled.

## Required local checks

Before committing meaningful implementation changes, run:

```bash
just pre-commit
```

For faster iteration, individual checks may be run:

```bash
just fmt
just check
just unused
just audit
just test
```

## Rust checks

Rust formatting:

```bash
just fmt-rs
```

Rust linting:

```bash
just check-rs
```

Rust tests:

```bash
just test-rs
```

Rust coverage:

```bash
just test-cov-rs
```

Rust audit:

```bash
just audit-rs
```

Rust unused dependencies:

```bash
just unused-rs
```

## Frontend checks

Frontend formatting:

```bash
just fmt-js
```

Frontend type/lint checks:

```bash
just check-js
```

Frontend tests:

```bash
just test-js
```

Frontend coverage:

```bash
just test-cov-js
```

Frontend audit:

```bash
just audit-js
```

Frontend unused dependencies:

```bash
just unused-js
```

## Documentation checks

Generated docs should build through:

```bash
just docs-all
```

Expected generated documentation categories:

- mdBook user/developer guide;
- backend Rust API docs;
- frontend TypeScript API docs;
- Svelte/Storybook UI docs.

## CI acceptance baseline

The CI quality baseline should include:

```bash
just ci-test
```

For release or docs publication, CI should also build docs and application artifacts as needed.

## Coverage expectations

The project should prefer meaningful coverage over arbitrary percentages.

Critical logic requiring tests:

- result classification;
- manifest/check comparison;
- SQLite persistence;
- CSV export;
- path handling;
- job cancellation state transitions.

## Error-handling expectations

Implementation code should prefer explicit errors over panics.

A good error type should preserve:

- operation being performed;
- affected path if applicable;
- lower-level error source if available;
- whether the error is job-fatal or entry-level.

## Documentation expectations

Public Rust items should have useful docs.

Developer documentation should explain architecture decisions, not merely restate code.

User-facing documentation should explain workflows and reports without requiring programming knowledge.

## Acceptance criteria

The quality-gates documentation is accepted when:

- expected Rust and frontend standards are documented;
- local check commands are documented;
- audit and unused-dependency expectations are documented;
- testing categories are documented;
- documentation generation is included in the quality model;
- future implementation work has a clear pre-commit and CI baseline.
