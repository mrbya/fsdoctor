# AGENTS.md

## Workflow
- Trust `justfile` over `README.md`; the README is still mostly placeholder text.
- First-time bootstrap is `just init`. It installs nightly Rust tooling, Tauri CLI, `cargo-xwin`, `cargo-nextest`, `cargo-udeps`, `cargo-llvm-cov`, `mdbook`, Playwright browsers, copies `env.example` to `.env`, and installs pre-commit hooks.
- Run the app with `just dev`. Tauri expects the frontend dev server on fixed port `1420` and HMR on `1421`.
- Before finishing meaningful code changes, run `just pre-commit`. It is mutating: it formats JS and Rust and reindexes `README.md` via `just index`.
- CI-style verification is `just ci-test` and `just ci-build`.

## Focused Checks
- Fast mixed checks: `just check`, `just test`, `just fmt`.
- Frontend only: `just check-js`, `just test-js`, `just fmt-js`, `pnpm test -- src/lib/api/index.test.ts`.
- Rust only: `just check-rs`, `just test-rs`, `just fmt-rs -- --check`.
- For a single Rust test, bypass `just` and run from `src-tauri/`: `SQLX_OFFLINE=true cargo nextest run -p fsdoctor-core <filter>`.
- Rust fmt and unused-dependency checks require nightly because the repo recipes use `cargo +nightly fmt` and `cargo +nightly udeps`.

## Backend Rustdoc And Lints
- Do not add new `#[allow(...)]` attributes just to silence clippy; fix the warning unless an existing local test pattern clearly applies.
- When adding or rewriting docs, match `docs/dev/rustdoc-style.md`.

## Architecture
- Real frontend entrypoint is `src/routes/+page.svelte`, which just mounts `src/lib/components/AppShell.svelte`; the app currently switches views inside that shell instead of using multiple routes.
- This frontend is intentionally SPA-only: `src/routes/+layout.ts` sets `ssr = false`, and `svelte.config.js` uses `adapter-static` with `fallback: "index.html"`.
- Keep all frontend Tauri IPC in `src/lib/api/index.ts`. Views and stores are expected to use those wrappers instead of calling `invoke()` directly.
- Keep durable integrity logic in `src-tauri/crates/fsdoctor-core`. The Tauri crate in `src-tauri/src/` is for command registration, job orchestration, DTOs, and event emission.
- Long-running backend work is job-based. Current manifest generation flow is wired through `src-tauri/src/commands.rs`, `src-tauri/src/handlers.rs`, `src-tauri/src/state.rs`, and `src/lib/stores/manifestGeneration.svelte.ts`.

## Data And Docs
- SQLite schema changes live in `src-tauri/migrations/`.
- Rust tests run with `SQLX_OFFLINE=true` in repo recipes.
- Docs source lives in `docs/book/` and `docs/dev/`; generated docs output goes to `docs-page/`. Rebuild docs with `just docs`.

## Testing And Storybook Quirks
- Frontend tests rely on `src/tests/setup.ts`, which polyfills WebCrypto and clears Tauri IPC mocks after each test.
- Storybook aliases Tauri modules to browser mocks from `src/stories/__mocks__/`, so UI work that touches Tauri-facing components should keep Storybook scenarios working.
- Vitest includes a Storybook browser project via Playwright; `just init` installs the required browser binaries.

## Generated Outputs
- Do not hand-edit generated artifacts in `build/`, `docs-page/`, `.svelte-kit/`, or `src-tauri/target/`.

<!-- BACKLOG.MD MCP GUIDELINES START -->

<CRITICAL_INSTRUCTION>

## BACKLOG WORKFLOW INSTRUCTIONS

This project uses Backlog.md MCP for all task and project management activities.

**CRITICAL GUIDANCE**

- If your client supports MCP resources, read `backlog://workflow/overview` to understand when and how to use Backlog for this project.
- If your client only supports tools or the above request fails, call `backlog.get_workflow_overview()` tool to load the tool-oriented overview (it lists the matching guide tools).

- **First time working here?** Read the overview resource IMMEDIATELY to learn the workflow
- **Already familiar?** You should have the overview cached ("## Backlog.md Overview (MCP)")
- **When to read it**: BEFORE creating tasks, or when you're unsure whether to track work

These guides cover:
- Decision framework for when to create tasks
- Search-first workflow to avoid duplicates
- Links to detailed guides for task creation, execution, and finalization
- MCP tools reference

You MUST read the overview resource to understand the complete workflow. The information is NOT summarized here.

</CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD MCP GUIDELINES END -->
