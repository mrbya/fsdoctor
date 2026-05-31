<script lang="ts">
  type StatusTone = "neutral" | "success" | "warning" | "danger" | "info";

  let {
    label,
    tone = "neutral",
  }: {
    label: string;
    tone?: StatusTone;
  } = $props();
</script>

<span class="badge tone-{tone}">{label}</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    width: fit-content;
    border: 1px solid var(--fd-color-border);
    border-radius: 999px;
    padding: 0.2rem 0.55rem;
    color: var(--fd-color-text-muted);
    background: var(--fd-color-bg-raised);
    font-size: 0.85rem;
    line-height: 1.2;
  }

  .tone-success {
    color: var(--fd-color-success);
    border-color: color-mix(in srgb, var(--fd-color-success), transparent 45%);
  }

  .tone-warning {
    color: var(--fd-color-warning);
    border-color: color-mix(in srgb, var(--fd-color-warning), transparent 45%);
  }

  .tone-danger {
    color: var(--fd-color-danger);
    border-color: color-mix(in srgb, var(--fd-color-danger), transparent 45%);
  }

  .tone-info {
    color: var(--fd-color-info);
    border-color: color-mix(in srgb, var(--fd-color-info), transparent 45%);
  }
</style>
