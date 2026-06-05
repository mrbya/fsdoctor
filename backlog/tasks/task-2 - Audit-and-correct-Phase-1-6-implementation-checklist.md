---
id: TASK-2
title: Audit and correct Phase 1-6 implementation checklist
status: In Progress
assignee:
  - OpenCode
created_date: '2026-06-05 13:23'
updated_date: '2026-06-05 13:26'
labels:
  - docs
  - audit
milestone: MVP Polish
dependencies: []
documentation:
  - docs/dev/implementation_plan.md
priority: high
ordinal: 1100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Review docs/dev/implementation_plan.md against the current codebase for Phases 1 through 6. Verify each checked item is actually implemented and tested, correct stale or inaccurate checklist state, and leave concise notes on incomplete items that are intentionally deferred or still not implemented.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Check Backup clearly communicates project-open state missing-manifest state live phase current path counters final summary and cancellation state with friendly labels
- [ ] #2 Shared UI polish improves spacing section hierarchy summary grids empty states and error details presentation across the MVP views touched by this work
- [ ] #3 Accessibility basics are improved for labels status readability and user-facing error disclosure
- [ ] #4 Storybook stories are updated if shared components are materially changed
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Verify Phase 1 shell/navigation/theme/command-boundary items against current frontend and architecture.
2. Verify Phase 2 create/open workflow, DB-inside-root warning, friendly errors, and database tests.
3. Verify Phase 3 and 4 scanner/hash behavior against core implementations and tests.
4. Verify Phase 5 and 6 manifest/integrity persistence, cancellation, and UI progress visibility against current backend wiring and views.
5. Update docs/dev/implementation_plan.md to reflect only confirmed behavior, adding concise notes where an item remains incomplete.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Verified during audit: AppShell provides navigable top-level views without route changes, and CreateManifest shows a readable database-inside-backup safety warning. Manifest generation progress visibility is still incomplete because only the finished event is wired to the frontend.
<!-- SECTION:NOTES:END -->
