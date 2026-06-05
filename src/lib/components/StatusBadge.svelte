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
    padding: 0.18rem 0.5rem;
    color: var(--text-muted);
    background: color-mix(in srgb, var(--bg-raised) 78%, transparent);
    font-size: var(--font-size-sm);
    letter-spacing: 0.02em;
    line-height: 1.2;
  }

  .tone-success {
    color: var(--success);
    border-color: color-mix(in srgb, var(--success) 40%, var(--border));
    background: color-mix(in srgb, var(--success) 10%, var(--bg-raised));
  }

  .tone-warning {
    color: var(--warning);
    border-color: color-mix(in srgb, var(--warning) 40%, var(--border));
    background: color-mix(in srgb, var(--warning) 10%, var(--bg-raised));
  }

  .tone-danger {
    color: var(--danger);
    border-color: color-mix(in srgb, var(--danger) 40%, var(--border));
    background: color-mix(in srgb, var(--danger) 10%, var(--bg-raised));
  }

  .tone-info {
    color: var(--info);
    border-color: color-mix(in srgb, var(--info) 40%, var(--border));
    background: color-mix(in srgb, var(--info) 10%, var(--bg-raised));
  }
</style>
