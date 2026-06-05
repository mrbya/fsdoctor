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
    border-radius: var(--radius-xl);
    padding: var(--space-lg);
    background: linear-gradient(
      180deg,
      color-mix(in srgb, var(--accent) 6%, var(--bg-raised)) 0%,
      var(--bg-muted) 100%
    );
    box-shadow: inset 0 1px 0 rgb(255 255 255 / 3%);
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
  }

  p {
    margin-top: var(--space-sm);
    color: var(--text-muted);
  }
</style>
