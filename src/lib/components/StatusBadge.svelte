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
    border: 1px solid var(--border);
    border-radius: 999px;
    padding: 0.2rem 0.55rem;
    color: var(--text-muted);
    background: var(--bg-raised);
    font-size: 0.85rem;
    line-height: 1.2;
  }

  .tone-success {
    color: var(--success);
    border-color: color-mix(in srgb, var(--success), transparent 45%);
  }

  .tone-warning {
    color: var(--warning);
    border-color: color-mix(in srgb, var(--warning), transparent 45%);
  }

  .tone-danger {
    color: var(--danger);
    border-color: color-mix(in srgb, var(--danger), transparent 45%);
  }

  .tone-info {
    color: var(--info);
    border-color: color-mix(in srgb, var(--info), transparent 45%);
  }
</style>
