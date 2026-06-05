<script lang="ts">
  import type { Snippet } from "svelte";

  type CardTone = "default" | "success" | "warning" | "danger" | "info";

  let {
    title,
    children,
    tone = "default",
  }: {
    title?: string;
    children: Snippet;
    tone?: CardTone;
  } = $props();
</script>

<section class="card tone-{tone}">
  {#if title !== undefined}
    <div class="card-header">
      <h2>{title}</h2>
    </div>
  {/if}

  <div class="card-body">
    {@render children()}
  </div>
</section>

<style>
  .card {
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .card-header {
    padding: 0.65rem 0.9rem;
    border-bottom: 1px solid var(--border);
    background: color-mix(in srgb, var(--bg-raised) 72%, transparent);
  }

  .card-header h2 {
    margin: 0;
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .card-body {
    display: grid;
    gap: var(--space-md);
    padding: 0.9rem;
  }

  .tone-success {
    border-color: color-mix(in srgb, var(--success) 32%, var(--border));
  }

  .tone-warning {
    border-color: color-mix(in srgb, var(--warning) 32%, var(--border));
    background: var(--bg-warning-soft);
  }

  .tone-danger {
    border-color: color-mix(in srgb, var(--danger) 35%, var(--border));
    background: var(--bg-danger-soft);
  }

  .tone-info {
    border-color: color-mix(in srgb, var(--info) 35%, var(--border));
  }
</style>
