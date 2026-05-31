<script lang="ts">
  import type { Snippet } from "svelte";

  type CardTone = "default" | "success" | "warning" | "danger" | "info";

  let {
    children,
    tone = "default",
  }: {
    children: Snippet;
    tone?: CardTone;
  } = $props();
</script>

<section class="card tone-{tone}">
  {@render children()}
</section>

<style>
  .card {
    border: 1px solid var(--fd-color-border);
    border-radius: var(--fd-radius-lg);
    padding: var(--fd-space-lg);
    background: var(--fd-color-bg-muted);
    box-shadow: var(--fd-shadow-raised);
  }

  .tone-success {
    border-color: var(--fd-color-success);
  }

  .tone-warning {
    border-color: var(--fd-color-warning);
  }

  .tone-danger {
    border-color: var(--fd-color-danger);
  }

  .tone-info {
    border-color: var(--fd-color-info);
  }
</style>
