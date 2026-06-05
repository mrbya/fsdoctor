---
id: TASK-1.2
title: Polish integrity-check and MVP layout UX
status: Done
assignee:
  - OpenCode
created_date: '2026-06-05 13:23'
updated_date: '2026-06-05 13:42'
labels:
  - ui
  - frontend
  - integrity
milestone: MVP Polish
dependencies:
  - TASK-1.1
documentation:
  - docs/dev/implementation_plan.md
  - docs/dev/reference-projects/plantracker
parent_task_id: TASK-1
priority: high
ordinal: 1300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve the Check Backup view and shared MVP layout quality so progress, summaries, empty states, and user-facing errors are more readable and accessible while staying within the existing SPA shell and current architecture. Avoid Phase 7 report history/export scope.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Check Backup clearly communicates project-open state missing-manifest state live phase current path counters final summary and cancellation state with friendly labels
- [x] #2 Shared UI polish improves spacing section hierarchy summary grids empty states and error details presentation across the MVP views touched by this work
- [x] #3 Accessibility basics are improved for labels status readability and user-facing error disclosure
- [x] #4 Storybook stories are updated if shared components are materially changed
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Use the audit results and manifest progress work to tighten the Create Manifest and Check Backup layouts with clearer section hierarchy, summary/counter presentation, accessible status/error copy, and responsive shared styling without changing the SPA navigation model.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Missing-manifest errors are now presented with user-facing copy instead of raw backend wording. Shared component changes remain small and preserve the existing SPA shell architecture.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Reworked the Check Backup UI to surface project-open state, missing-manifest messaging, live counters, current phase/path, and a distinct final summary.

Improved shared MVP polish with better spacing, summary card hierarchy, accessible icon-only navigation labels, friendlier error presentation, and updated ProgressPanel Storybook coverage.
<!-- SECTION:FINAL_SUMMARY:END -->
