---
id: TASK-4
title: Update Storybook coverage and verification for the UI refresh
status: Done
assignee:
  - OpenCode
created_date: '2026-06-05 19:04'
updated_date: '2026-06-05 19:38'
labels:
  - ui
  - storybook
  - verification
milestone: MVP UI Polish
dependencies:
  - TASK-3
  - TASK-5
documentation:
  - AGENTS.md
priority: medium
ordinal: 3300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Keep the UI refresh supported by concise component stories and project validation. Update stories that materially change, add targeted stories if new reusable presentation components are introduced, and run the established checks required by the touched files.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Storybook stories remain aligned with the updated shared components and load the global theme correctly
- [x] #2 Relevant frontend and repository verification commands are run successfully, with Rust checks included only if backend files change
- [x] #3 Backlog notes capture deferred UI ideas and final task status reflects completed verified work
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update existing stories for components whose visual contracts change materially during TASK-3 and TASK-5.
2. Add a focused story only if a new reusable presentation component is introduced for progress or summaries.
3. Run the established frontend and repo-level validation commands that match the touched files, including Storybook-related checks when story files change.
4. Record deferred UI ideas in backlog notes and finalize all task statuses once checks pass.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Storybook build still emits the existing JetBrains Mono font asset resolution warnings from the current font CSS setup, but the build succeeds and the UI stories render. Deferred idea: clean up packaged font asset paths in a follow-up if the team wants warning-free Storybook builds.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Updated Storybook coverage for the refreshed component surface by adjusting button, card, page-header, and status-badge stories and by adding a focused `MetricGrid` story for the new progress/summary primitive. Verification covered frontend formatting, Svelte type checking, dependency/audit checks, frontend tests, Storybook build, and the repository-wide `just check` and `just test` commands. All requested checks passed.
<!-- SECTION:FINAL_SUMMARY:END -->
