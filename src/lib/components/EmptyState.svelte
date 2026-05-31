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
    place-items: start;
    gap: var(--fd-space-sm);
    border: 1px dashed var(--fd-color-border);
    border-radius: var(--fd-radius-lg);
    padding: var(--fd-space-xl);
    color: var(--fd-color-text-muted);
    background: rgb(49 50 68 / 40%);
  }

  h2,
  p {
    margin: 0;
  }

  h2 {
    color: var(--fd-color-text);
  }

  .actions {
    margin-top: var(--fd-space-sm);
  }
</style>
