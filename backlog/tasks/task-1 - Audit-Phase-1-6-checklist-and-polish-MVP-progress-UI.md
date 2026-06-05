---
id: TASK-1
title: Audit Phase 1-6 checklist and polish MVP progress UI
status: Done
assignee:
  - OpenCode
created_date: '2026-06-05 13:23'
updated_date: '2026-06-05 13:42'
labels:
  - ui
  - backend
  - docs
  - audit
milestone: MVP Polish
dependencies: []
documentation:
  - docs/dev/implementation_plan.md
  - docs/dev/reference-projects/plantracker
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Audit the current FSDoctor implementation against the Phase 1-6 acceptance criteria in docs/dev/implementation_plan.md, correct stale checklist items based on actual behavior, and close remaining MVP polish gaps around manifest generation progress, integrity-check progress/readability, and general UI quality without redesigning the app architecture or starting Phase 7 history/export work.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Phase 1-6 acceptance criteria in docs/dev/implementation_plan.md are audited against the code and only checked when truly implemented
- [x] #2 Manifest generation exposes useful live progress from backend through the existing job/event model and the Create Manifest view renders it clearly
- [x] #3 Integrity check progress and final summary are readable in the Check Backup view with friendly labels and visible cancellation state
- [x] #4 General MVP UI polish improves spacing hierarchy empty states and error presentation without introducing Phase 7 report-history/export features
- [x] #5 Relevant stories are updated when shared UI components materially change
- [x] #6 Project checks covering frontend and Rust pass or any blocker is documented in the task notes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Audit docs/dev/implementation_plan.md Phase 1-6 acceptance criteria against current frontend, Tauri, core, and test coverage; update only items that are demonstrably implemented and leave concise notes on incomplete items.
2. Add live manifest-generation progress using fsdoctor-core progress snapshots, Tauri DTO/event emission, and frontend store listeners while preserving the current job/cancellation architecture.
3. Polish Create Manifest and Check Backup views plus any small shared UI components needed for readable live progress, final summaries, empty states, and user-facing errors.
4. Update Storybook only if shared components materially change.
5. Run the established frontend/Rust/repo checks, then mark acceptance criteria complete only for verified work and document any intentional deferrals.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Intentionally deferred roadmap items left unchecked: unreadable-entry scanner test coverage, deterministic changed-during-hash test coverage, and post-restart integrity summary/history loading which remains Phase 7 scope.

`just pre-commit` passed after the implementation changes. The only remaining audit noise is the existing allowed `cargo audit` warnings from upstream Tauri/GTK dependency advisories.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Updated the Phase 1-6 roadmap checklist to match the verified implementation and test coverage, including leaving still-missing deterministic coverage explicitly unchecked with concise notes.

Completed MVP polish for manifest generation progress, integrity-check readability, shared layout/accessibility improvements, and verification through the repo quality gates.
<!-- SECTION:FINAL_SUMMARY:END -->
