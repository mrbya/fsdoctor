<script lang="ts">
  let {
    eyebrow,
    title = "No job running",
    description,
  }: {
    eyebrow?: string;
    title?: string;
    description?: string;
  } = $props();
</script>

<div class="progress-panel">
  {#if eyebrow !== undefined}
    <p class="eyebrow">{eyebrow}</p>
  {/if}

  <h2>{title}</h2>

  {#if description !== undefined}
    <p>{description}</p>
  {/if}
</div>

<style>
  .progress-panel {
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 0.9rem;
    background: var(--bg-soft);
  }

  .eyebrow,
  h2,
  p {
    margin: 0;
  }

  .eyebrow {
    color: var(--accent);
    font-size: var(--font-size-sm);
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  h2 {
    margin-top: var(--space-2xs);
    font-size: var(--font-size-xl);
    line-height: 1.3;
  }

  p {
    margin-top: var(--space-xs);
    color: var(--text-muted);
  }
</style>
