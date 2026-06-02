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
    display: grid;
    gap: var(--fd-space-xs);
    color: var(--fd-color-text-muted);
  }

  .row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--fd-space-xs);
  }

  input {
    min-width: 0;
    border: 1px solid var(--fd-color-border);
    border-radius: var(--fd-radius-md);
    padding: 0.7rem;
    color: var(--fd-color-text);
    background: var(--fd-color-bg-elevated);
  }
</style>
