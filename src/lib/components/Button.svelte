<script lang="ts">
  import type { Snippet } from "svelte";

  type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";

  let {
    children,
    type = "button",
    variant = "primary",
    disabled = false,
    onclick,
  }: {
    children: Snippet;
    type?: "button" | "submit" | "reset";
    variant?: ButtonVariant;
    disabled?: boolean;
    onclick?: (even: MouseEvent) => void;
  } = $props();
</script>

<button class="button variant-{variant}" {type} {disabled} {onclick}>
  {@render children()}
</button>

<style>
  .button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--fd-space-xs);
    min-height: 2.5rem;
    border: 1px solid transparent;
    border-radius: var(--fd-radius-md);
    padding: 0.65rem 1rem;
    color: var(--fd-color-bg-elevated);
    background: var(--fd-color-accent);
    font: inherit;
    cursor: pointer;
    transition:
      border-color 120ms ease,
      background 120ms ease,
      opacity 120ms ease;
  }

  .button:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  .variant-primary {
    color: var(--ctp-crust);
    background: var(--fd-color-accent);
  }

  .variant-secondary {
    color: var(--fd-color-text);
    border-color: var(--fd-color-border);
    background: var(--fd-color-bg-raised);
  }

  .variant-ghost {
    color: var(--fd-color-text-muted);
    background: transparent;
  }

  .variant-danger {
    color: var(--ctp-crust);
    background: var(--fd-color-danger);
  }
</style>
