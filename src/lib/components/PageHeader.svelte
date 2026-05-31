<script lang="ts">
  let {
    eyebrow = "FsDoctor",
    title,
    description,
  }: {
    eyebrow?: string;
    title: string;
    description?: string;
  } = $props();
</script>

<header class="page-header">
  <p class="eyebrow">{eyebrow}</p>
  <h1>{title}</h1>

  {#if description !== undefined}
    <p class="description">{description}</p>
  {/if}
</header>

<style>
  .page-header {
    display: grid;
    gap: var(--fd-space-xs);
  }

  .eyebrow {
    margin: 0;
    color: var(--fd-color-accent);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  h1 {
    margin: 0;
    color: var(--fd-color-text);
    font-size: clamp(1.75rem, 4vw, 3rem);
  }

  .description {
    max-width: 72ch;
    margin: 0;
    color: var(--fd-color-text-muted);
    line-height: 1.6;
  }
</style>
