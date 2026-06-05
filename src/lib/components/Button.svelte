<script lang="ts">
  import type { Snippet } from "svelte";
  import Spinner from "./Spinner.svelte";

  type ButtonVariant = "primary" | "secondary" | "ghost" | "danger" | "success";

  let {
    children,
    type = "button",
    variant = "primary",
    disabled = false,
    loading = false,
    title,
    onclick,
  }: {
    children: Snippet;
    type?: "button" | "submit" | "reset";
    variant?: ButtonVariant;
    disabled?: boolean;
    loading?: boolean;
    title?: string;
    onclick?: (even: MouseEvent) => void;
  } = $props();
</script>

<button
  class="button variant-{variant}"
  {type}
  disabled={disabled || loading}
  {onclick}
  {title}
>
  {#if loading}
    <Spinner size="sm" />
  {/if}

  {@render children()}
</button>

<style>
  .button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    min-height: 2.25rem;
    padding: 0.45rem 0.85rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-family: var(--font);
    font-size: var(--font-size-base);
    font-weight: 500;
    line-height: 1.2;
    cursor: pointer;
    transition:
      background 0.15s,
      color 0.15s,
      border-color 0.15s,
      opacity 0.15s;
    white-space: nowrap;
  }

  .button:disabled {
    cursor: not-allowed;
    opacity: 0.55;
  }

  .variant-primary {
    background: var(--accent);
    color: var(--bg);
    border-color: var(--accent);
  }

  .variant-primary:hover:not(:disabled) {
    background: var(--accent-hover);
    border-color: var(--accent-hover);
  }

  .variant-secondary {
    color: var(--text);
    border-color: var(--border);
    background: var(--bg-raised);
  }

  .variant-secondary:hover:not(:disabled) {
    background: var(--bg-soft);
    border-color: var(--border-strong);
  }

  .variant-ghost {
    background: transparent;
    color: var(--text);
    border-color: var(--border);
  }

  .variant-ghost:hover:not(:disabled) {
    background: color-mix(in srgb, var(--bg-raised) 72%, transparent);
    border-color: var(--border-strong);
  }

  .variant-danger {
    background: var(--danger);
    color: var(--bg);
    border-color: var(--danger);
  }

  .variant-danger:hover:not(:disabled) {
    opacity: 0.88;
  }

  .variant-success {
    background: var(--success);
    color: var(--bg);
    border-color: var(--success);
  }

  .variant-success:hover:not(:disabled) {
    opacity: 0.88;
  }
</style>
