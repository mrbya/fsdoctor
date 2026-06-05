<script lang="ts">
  import { Card, PageHeader, StatusBadge } from "$lib/components";
</script>

<div class="view">
  <PageHeader
    title="Settings"
    description="Application preferences are intentionally deferred while the core manifest and integrity workflows are being polished."
  />

  <Card title="MVP scope">
    <div class="stack">
      <StatusBadge label="Not implemented" tone="info" />
      <h2>Settings are not available yet</h2>
      <p>
        Likely follow-up options include hashing parallelism, advanced path
        handling, and report export preferences.
      </p>
    </div>
  </Card>
</div>

<style>
  .view {
    display: grid;
    gap: var(--space-lg);
  }

  .stack {
    display: grid;
    gap: var(--space-sm);
  }

  h2,
  p {
    margin: 0;
  }

  p {
    color: var(--text-muted);
  }
</style>
