<script lang="ts">
  import { Card, EmptyState, PageHeader, ResultTable } from "$lib/components";
</script>

<div class="view">
  <PageHeader
    title="Reports"
    description="Report history is planned for a later phase. This placeholder stays intentionally minimal for the MVP."
  />

  <Card title="Status">
    <EmptyState
      title="Report history is not implemented yet"
      description="Phase 7 work such as report history, filtering, pagination, and export is intentionally deferred."
    />
  </Card>

  <ResultTable
    message="A future report table will appear here once Phase 7 work begins."
  />
</div>

<style>
  .view {
    display: grid;
    gap: var(--space-lg);
  }
</style>
