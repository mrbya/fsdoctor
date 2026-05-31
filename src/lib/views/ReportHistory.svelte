<script lang="ts">
  import { EmptyState, PageHeader, ResultTable } from "$lib/components";
</script>

<div class="view">
  <PageHeader
    title="Reports"
    description="Review previous manifest generation and integrity check reports."
  />

  <EmptyState
    title="No reports yet"
    description="Reports will appear here after manifest generation and integrity checks are implemented."
  />

  <ResultTable />
</div>

<style>
  .view {
    display: grid;
    gap: var(--fd-space-lg);
  }
</style>
