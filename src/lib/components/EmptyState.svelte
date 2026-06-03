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
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 3rem 1rem;
    color: var(--text-muted);
    text-align: center;
    opacity: 0.5;
  }

  h2,
  p {
    margin: 0;
    font-size: var(--font-size-base);
  }

  h2 {
    color: var(--text);
  }

  .actions {
    margin-top: var(--space-sm);
  }
</style>
