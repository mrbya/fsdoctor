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
  <span class="label">{label}</span>
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
    gap: var(--space-xs);
  }

  .label {
    color: var(--text-muted);
  }

  .row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: var(--space-xs);
    align-items: start;
  }

  input {
    min-width: 0;
  }

  @media (max-width: 42rem) {
    .row {
      grid-template-columns: 1fr;
    }
  }
</style>
