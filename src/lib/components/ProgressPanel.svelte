<script lang="ts">
  let {
    title = "No job running",
    description,
  }: {
    title?: string;
    description?: string;
  } = $props();
</script>

<div class="progress-panel">
  <h2>{title}</h2>

  {#if description !== undefined}
    <p>{description}</p>
  {/if}
</div>

<style>
  .progress-panel {
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    background: var(--bg-muted);
  }

  h2,
  p {
    margin: 0;
  }

  p {
    margin-top: var(--space-xs);
    color: var(--text-muted);
  }
</style>
