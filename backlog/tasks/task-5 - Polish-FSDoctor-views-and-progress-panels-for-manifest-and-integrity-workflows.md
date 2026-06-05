---
id: TASK-5
title: Polish FSDoctor views and progress panels for manifest and integrity workflows
status: Done
assignee:
  - OpenCode
created_date: '2026-06-05 19:04'
updated_date: '2026-06-05 19:38'
labels:
  - ui
  - frontend
  - views
milestone: MVP UI Polish
dependencies:
  - TASK-3
references:
  - docs/dev/reference-projects/plantracker
documentation:
  - AGENTS.md
priority: high
ordinal: 3200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Restructure the main FSDoctor views so project setup, long-running jobs, progress, summaries, and placeholders are easier to scan. Keep the current workflows intact while making the desktop layout feel calmer and more app-like. Use reusable progress and summary building blocks only where they reduce duplication.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Create Manifest separates project setup, warnings, job controls, live progress, and final summary into a compact readable layout
- [x] #2 Check Backup presents project status, check controls, live progress, and final report summary clearly without oversized raw counter blocks
- [x] #3 Dashboard, Reports placeholder, and Settings placeholder align with the updated visual language and clearly communicate current MVP scope
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Revisit Dashboard, Create Manifest, Check Backup, Reports placeholder, and Settings placeholder after TASK-3 establishes the visual system.
2. Break Create Manifest into clearer sections for project setup, warnings, job controls, live progress, and final summary.
3. Rework Check Backup around a compact action panel plus clearer live progress and final summary sections.
4. Align placeholders and dashboard content with the calmer shell and spacing rhythm.
5. Run frontend validation focused on these views once complete.

Implemented the plan with a reusable MetricGrid to reduce duplication across the two long-running workflow views.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Completed the workflow layout refresh using the new shared visual system. Deferred ideas: richer Phase 7 report tables with sticky columns and filtering controls, and a possible shared workflow-section helper if additional job-driven views adopt the same pattern later.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Reworked the FSDoctor workflow screens around calmer app-style sections instead of large undifferentiated blocks. Create Manifest and Check Backup now separate project context, actions, live job state, current path, compact metric grids, final summaries, and technical error details. Dashboard, Reports, and Settings were also aligned with the new visual system while keeping placeholders explicitly in MVP scope. A reusable `MetricGrid` component was introduced to keep the progress and summary layouts consistent across manifest generation and integrity checks.
<!-- SECTION:FINAL_SUMMARY:END -->
