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
    gap: 0.4rem;
    padding: 0.45rem 1rem;
    border: 1px solid transparent;
    border-radius: var(--radius);
    font-family: var(--font);
    font-size: var(--font-size-base);
    font-weight: 500;
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
    opacity: 0.6;
  }

  .variant-primary {
    background: var(--accent);
    color: var(--bg);
    border-color: var(--accent);
  }

  .variant-secondary {
    color: var(--text);
    border-color: var(--border);
    background: var(--bg-raised);
  }

  .variant-ghost {
    background: transparent;
    color: var(--text);
    border-color: var(--border);
  }

  .variant-danger {
    background: var(--danger);
    color: var(--bg);
    border-color: var(--danger);
  }

  .variant-danger:hover:not(:disabled) {
    opacity: 0.85;
  }
</style>
