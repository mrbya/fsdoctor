<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    title,
    description,
    children,
  }: {
    title: string;
    description?: string;
    children?: Snippet;
  } = $props();
</script>

<div class="empty-state">
  <h2>{title}</h2>

  {#if description !== undefined}
    <p>{description}</p>
  {/if}

  {#if children !== undefined}
    <div class="actions">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .empty-state {
    display: grid;
    gap: var(--space-xs);
    padding: 1rem;
    border: 1px dashed var(--border);
    border-radius: var(--radius);
    background: var(--bg-soft);
    color: var(--text-muted);
    text-align: left;
  }

  h2,
  p {
    margin: 0;
  }

  h2 {
    color: var(--text);
    font-size: var(--font-size-lg);
  }

  .actions {
    margin-top: var(--space-xs);
  }
</style>
