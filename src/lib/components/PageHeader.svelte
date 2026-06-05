<script lang="ts">
  let {
    eyebrow,
    title,
    description,
  }: {
    eyebrow?: string;
    title: string;
    description?: string;
  } = $props();
</script>

<header class="page-header">
  {#if eyebrow !== undefined}
    <p class="eyebrow">{eyebrow}</p>
  {/if}

  <h1>{title}</h1>

  {#if description !== undefined}
    <p class="description">{description}</p>
  {/if}
</header>

<style>
  .page-header {
    display: grid;
    gap: var(--space-2xs);
  }

  .eyebrow {
    margin: 0;
    color: var(--accent);
    text-transform: uppercase;
    font-size: var(--font-size-xs);
    letter-spacing: 0.08em;
  }

  h1 {
    margin: 0;
    color: var(--text);
    font-size: var(--font-size-2xl);
    font-weight: 600;
    line-height: 1.25;
  }

  .description {
    max-width: 68ch;
    margin: 0;
    color: var(--text-muted);
    line-height: 1.5;
  }
</style>
