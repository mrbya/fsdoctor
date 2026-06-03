<script lang="ts">
  import Button from "./Button.svelte";

  let {
    label,
    value = $bindable(""),
    placeholder,
    buttonLabel = "Browse",
    disabled = false,
    onbrowse,
  }: {
    label: string;
    value?: string;
    placeholder?: string;
    buttonLabel?: string;
    disabled?: boolean;
    onbrowse?: () => void;
  } = $props();
</script>

<label class="field">
  <span class="row">{label}</span>
  <div class="row">
    <input bind:value {placeholder} {disabled} />
    <Button
      variant="secondary"
      disabled={disabled || onbrowse === undefined}
      onclick={onbrowse}
    >
      {buttonLabel}
    </Button>
  </div>
</label>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    color: var(--text-muted);
  }

  .row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--space-xs);
  }

  input {
    width: 100%;
    padding: 0.45rem 0.75rem;
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text);
    font-family: var(--font);
    font-size: var(--font-size-base);
    transition: border-color 0.15s;
  }

  input:hover {
    border-color: var(--text-muted);
  }

  input:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>
