<script lang="ts">
  import { Card, PageHeader, StatusBadge } from "$lib/components";
</script>

<div class="view">
  <PageHeader
    title="Settings"
    description="Application preferences and advanced options will live here."
  />

  <Card>
    <StatusBadge label="MVP" tone="info" />
    <h2>Settings are not implemented yet</h2>
    <p>
      Future options may include hashing parallelism, advanced path handling,
      and report export preferences.
    </p>
  </Card>
</div>

<style>
  .view {
    display: grid;
    gap: var(--fd-space-lg);
  }

  h2,
  p {
    margin: 0;
  }

  p {
    margin-top: var(--fd-space-xs);
    color: var(--fd-color-text-muted);
  }
</style>
