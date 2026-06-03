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
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .tone-success {
    border-color: var(--success);
  }

  .tone-warning {
    border-color: var(--warning);
  }

  .tone-danger {
    border-color: var(--danger);
  }

  .tone-info {
    border-color: var(--info);
  }
</style>
