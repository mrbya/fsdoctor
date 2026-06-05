---
id: TASK-1.1
title: Add live manifest-generation progress and summary polish
status: Done
assignee:
  - OpenCode
created_date: '2026-06-05 13:23'
updated_date: '2026-06-05 13:42'
labels:
  - ui
  - backend
  - manifest
milestone: MVP Polish
dependencies: []
documentation:
  - docs/dev/implementation_plan.md
parent_task_id: TASK-1
priority: high
ordinal: 1200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve manifest generation to surface useful live progress through the existing backend job/event model and render it cleanly in the Create Manifest view. Progress should prefer real counters over fake percentages and include current operation and path when available.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Manifest generation emits serializable progress snapshots that include current phase and available counters without introducing Tauri dependencies into fsdoctor-core
- [x] #2 Frontend manifest-generation state stores the latest progress snapshot and preserves existing final event handling
- [x] #3 Create Manifest clearly shows live progress counters current path cancellation state and final summary
- [x] #4 Manifest generation cancellation remains functional and visible to the user
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
After the checklist audit confirms the current gap, add a manifest progress callback in fsdoctor-core, map it to small serializable DTOs in src-tauri, emit manifest-generation-progress events from the Tauri handler, store the latest snapshot in the Svelte store, and render live counters and final summary in Create Manifest.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Verified with `just check-rs`, `just test-rs`, `just check`, `just test`, and `just pre-commit`. Manifest progress is now visible live instead of only at completion.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added live manifest-generation progress end-to-end from fsdoctor-core through Tauri events into the Svelte store and Create Manifest UI.

Added core tests covering manifest progress snapshots and repeated manifest generation updating existing paths without duplicates.
<!-- SECTION:FINAL_SUMMARY:END -->
