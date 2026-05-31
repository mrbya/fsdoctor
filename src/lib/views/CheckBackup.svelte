<script lang="ts">
  import { Card, EmptyState, PageHeader, ProgressPanel } from "$lib/components";
</script>

<div class="view">
  <PageHeader
    title="Check backup"
    description="Verify a backup folder against an existing FSDoctor integrity record."
  />

  <Card>
    <EmptyState
      title="Integrity checks are not implemented yet"
      description="This view is prepared for the manifest-checking phase."
    />
  </Card>

  <ProgressPanel
    title="No check running"
    description="Progress will appear here once integrity checks are implemented."
  />
</div>

<style>
  .view {
    display: grid;
    gap: var(--fd-space-lg);
  }
</style>
