# Documentation

This document defines the documentation strategy for `FSDoctor`.

The project uses one published documentation output directory that contains multiple generated documentation sites.
The book acts as the documentation portal, while Rustdoc, TypeDoc, and Storybook keep their native generated formats.

## Documentation goals

Documentation should serve four audiences:

1. end users who need to run backup checks;
2. contributors implementing backend logic;
3. contributors implementing frontend/API logic;
4. contributors designing or reviewing UI components.

No single documentation generator is ideal for all four audiences.
The project therefore uses a combined static output tree.

## Documentation categories

### User and high-level project docs

Generated with mdBook (this book).

Content:

- introduction;
- user guide;
- creating integrity records;
- checking backups;
- understanding reports;
- development specification;
- architecture;
- data model;
- pipeline behavior;
- UI/UX guidelines;
- quality gates;
- documentation strategy.

### Backend Rust API docs

Generated with `cargo doc`.

Content:

- backend crate APIs;
- private implementation docs where configured;
- core domain types;
- database APIs;
- scanner/checker modules;
- report/export modules.

### Frontend TypeScript API docs

Generated with TypeDoc.

Content:

- TypeScript command wrappers;
- event payload types;
- stores;
- API helper modules;
- frontend domain models.

### Svelte UI component docs

Generated with Storybook.

Content:

- reusable UI components;
- status badges;
- report cards;
- progress panels;
- result tables;
- important UI states.

## Published output layout

All generated docs should be published under:

```text
docs-page/
```

Ffinal layout:

```text
docs-page/
├── index.html
├── user-guide/
├── development/
└── dev/
    ├── backend-rustdoc/
    │   └── index.html
    ├── frontend-api/
    │   └── index.html
    └── frontend-ui/
        └── index.html
```

The mdBook output provides the top-level `index.html`.
Generated developer docs live below `docs-page/dev/`.

## Links to generated developer docs

The mdBook should link to generated developer docs rather than embedding them as chapters.

Links to generated docs are aggregated in [Generated dev docs](./development/generated-dev-docs.md)

## Local generation

Generate all documentation:

```bash
just docs-all
```

Generate only the mdBook:

```bash
just docs
```

Generate backend Rust API docs:

```bash
just docs-rs
```

Generate frontend TypeScript API docs:

```bash
just docs-api
```

Generate frontend UI docs:

```bash
just docs-ui
```

Serve or preview generated docs:

```bash
just docs-show
```

## mdBook role

The book is both user-facing and project-facing.

It should contain:

- user workflows;
- non-technical explanations;
- stable project decisions;
- architecture and design notes;
- links to generated API/component docs.

The book should not duplicate full generated API docs.

## Rustdoc role

Rustdoc is the source of truth for Rust APIs.

Rustdoc should answer:

- what does this module/type/function do?
- what invariants does it require?
- what errors can it return?
- how is it intended to be used?

Architecture-level explanations belong in the book.
API-level explanations belong in Rustdoc.

## TypeDoc role

TypeDoc is the source of truth for TypeScript APIs.

TypeDoc should document:

- Tauri command wrappers;
- event payload types;
- stores;
- helper functions;
- frontend data models.

Svelte component usage belongs in Storybook.

## Storybook role

Storybook is the source of truth for reusable UI component states.

Stories should cover:

- normal state;
- loading/running state;
- healthy result;
- warning result;
- error result;
- empty state;
- long path/content state.

Storybook should help verify that components remain usable under realistic data.

## Documentation ownership

When changing behavior, update documentation in the same change.

Examples:

- changing result classification requires updating the pipeline and report docs;
- changing database schema requires updating the data model docs;
- changing UI wording requires updating UI/UX docs;
- adding a command wrapper requires TypeDoc comments;
- adding a reusable component requires Storybook coverage.

## Documentation acceptance criteria

The documentation setup is accepted when:

- `just docs-all` produces a complete `docs-page/` tree;
- mdBook links to Rustdoc, TypeDoc, and Storybook outputs;
- user workflows are documented in the book;
- generated docs remain in their native formats;
- CI can publish `docs-page/` as a single static artifact.
