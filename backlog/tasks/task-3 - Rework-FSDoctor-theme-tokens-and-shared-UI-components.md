---
id: TASK-3
title: Rework FSDoctor theme tokens and shared UI components
status: In Progress
assignee:
  - OpenCode
created_date: '2026-06-05 19:04'
updated_date: '2026-06-05 19:05'
labels:
  - ui
  - frontend
  - components
milestone: MVP UI Polish
dependencies: []
references:
  - docs/dev/reference-projects/plantracker
documentation:
  - AGENTS.md
priority: high
ordinal: 3100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update the frontend theme and base components to use a calmer, more compact visual language closer to Plantracker. Cover semantic tokens, typography scale, shared surfaces, buttons, headers, forms, and supporting primitives such as status and empty states. Keep any remaining compatibility bridges only where needed to avoid a disruptive rewrite.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Theme tokens favor a consistent semantic system with compact typography, calmer spacing, and Catppuccin Mocha-inspired colors
- [ ] #2 PageHeader, Card, Button, StatusBadge, EmptyState, FilePickerRow, ResultTable, and related form styles match the updated visual direction
- [ ] #3 Sidebar and shell spacing remain balanced, icon-only controls keep accessible labels, and focus states stay visible
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Initialize or otherwise access the local plantracker reference files, then inspect its theme and shared UI components for token, spacing, typography, and surface patterns to adapt rather than copy.
2. Normalize FSDoctor theme tokens and global styles toward a compact Catppuccin Mocha-inspired desktop scale, keeping only minimal compatibility bridges for remaining fd-* usage.
3. Rework shared primitives including AppShell spacing, PageHeader, Card, Button, StatusBadge, EmptyState, FilePickerRow, ResultTable, and form/input styling so the shell and controls feel calmer and more consistent.
4. Verify that accessibility basics remain intact for focus, icon-only navigation buttons, disabled states, and status presentation.
5. Run the relevant frontend checks after component-level changes and capture any deferred design ideas in backlog notes.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Initial inspection complete. Current FSDoctor styles use mixed semantic and legacy fd-* tokens, oversized page headers, and relatively loud progress/card surfaces. The local plantracker reference directory is currently empty in the checked-out workspace, so implementation may require initializing the submodule before inspecting its files.
<!-- SECTION:NOTES:END -->
