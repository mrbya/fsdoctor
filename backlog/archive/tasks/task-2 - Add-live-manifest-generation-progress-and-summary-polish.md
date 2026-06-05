---
id: TASK-2
title: Add live manifest-generation progress and summary polish
status: To Do
assignee:
  - OpenCode
created_date: '2026-06-05 13:23'
labels:
  - ui
  - backend
  - manifest
milestone: MVP Polish
dependencies: []
documentation:
  - docs/dev/implementation_plan.md
priority: high
ordinal: 1200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve manifest generation to surface useful live progress through the existing backend job/event model and render it cleanly in the Create Manifest view. Progress should prefer real counters over fake percentages and include current operation and path when available.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Manifest generation emits serializable progress snapshots that include current phase and available counters without introducing Tauri dependencies into fsdoctor-core
- [ ] #2 Frontend manifest-generation state stores the latest progress snapshot and preserves existing final event handling
- [ ] #3 Create Manifest clearly shows live progress counters current path cancellation state and final summary
- [ ] #4 Manifest generation cancellation remains functional and visible to the user
<!-- AC:END -->
