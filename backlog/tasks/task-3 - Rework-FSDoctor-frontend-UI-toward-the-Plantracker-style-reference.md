---
id: TASK-3
title: Rework FSDoctor frontend UI toward the Plantracker style reference
status: Done
assignee:
  - OpenCode
created_date: '2026-06-05 19:04'
updated_date: '2026-06-05 19:38'
labels:
  - ui
  - frontend
  - design-system
milestone: MVP UI Polish
dependencies: []
references:
  - docs/dev/reference-projects/plantracker
documentation:
  - docs/dev/implementation_plan.md
  - AGENTS.md
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Refresh the FSDoctor desktop frontend so it feels compact, calm, and coherent, using the local Plantracker reference project as the primary design source once available. Preserve existing manifest-generation and integrity-check behavior, keep Tauri IPC inside src/lib/api/index.ts, keep the current navigation model, and avoid Phase 7 feature work. Track the redesign as coordinated subtasks so tokens, shared components, views, progress displays, Storybook coverage, and verification stay aligned.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Theme and shared UI styling are reworked toward a compact Plantracker-inspired desktop app style without regressing current functionality
- [x] #2 Create Manifest, Check Backup, Dashboard, Reports placeholder, and Settings placeholder use the updated visual system consistently
- [x] #3 Progress and summary surfaces for manifest generation and integrity checks are clearer, more compact, and remain truthful about job state
- [x] #4 Relevant Storybook stories and project tracking notes are updated, and required checks pass
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect the local plantracker reference files for token, spacing, typography, and component patterns to adapt rather than copy.
2. Normalize FSDoctor theme tokens and global styles toward a compact Catppuccin Mocha-inspired desktop scale, keeping only minimal compatibility bridges for remaining fd-* usage.
3. Rework shared primitives including AppShell spacing, PageHeader, Card, Button, StatusBadge, EmptyState, FilePickerRow, ResultTable, and form/input styling so the shell and controls feel calmer and more consistent.
4. Verify that accessibility basics remain intact for focus, icon-only navigation buttons, disabled states, and status presentation.
5. Run the relevant frontend checks after component-level changes and capture any deferred design ideas in backlog notes.

Executed the shared-component refresh first, then rebuilt the workflow views and Storybook coverage on top of that system.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Reference review confirmed that Plantracker's compact typography, simple borders, quieter surfaces, and narrow token system were the right direction for FSDoctor. Deferred ideas: richer Phase 7 report/history UX, a possible shared workflow section helper if more job views are added, and a separate cleanup of Storybook font asset warnings if desired.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Reworked the FSDoctor frontend into a more compact Plantracker-inspired desktop UI while preserving the existing manifest-generation and integrity-check behavior. The refresh normalized Catppuccin Mocha-style semantic tokens and global form/focus styles, tightened the page header scale, flattened cards and buttons, polished the sidebar shell, and introduced reusable Spinner and MetricGrid components. On top of that shared system, the main workflow views were restructured into calmer sections with clearer job state, live counters, current-path handling, final summaries, and explicitly scoped placeholders for not-yet-implemented areas. Storybook coverage was updated for the changed components and all requested validation commands passed.
<!-- SECTION:FINAL_SUMMARY:END -->
