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
    border: 1px solid var(--fd-color-border);
    border-radius: var(--fd-radius-lg);
    padding: var(--fd-space-lg);
    background: var(--fd-color-bg-muted);
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
