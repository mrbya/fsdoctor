<script lang="ts">
  type MetricTone = "default" | "success" | "warning" | "danger";

  type MetricItem = {
    label: string;
    value: string;
    tone?: MetricTone;
  };

  let {
    items,
  }: {
    items: MetricItem[];
  } = $props();
</script>

<dl class="metric-grid">
  {#each items as item (item.label)}
    <div
      class:tone-success={item.tone === "success"}
      class:tone-warning={item.tone === "warning"}
      class:tone-danger={item.tone === "danger"}
    >
      <dt>{item.label}</dt>
      <dd>{item.value}</dd>
    </div>
  {/each}
</dl>

<style>
  .metric-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(10.5rem, 1fr));
    gap: var(--space-sm);
  }

  .metric-grid div {
    display: grid;
    gap: var(--space-2xs);
    padding: 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-soft);
    min-width: 0;
  }

  dt {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
  }

  dd {
    margin: 0;
    color: var(--text);
    font-size: var(--font-size-lg);
    line-height: 1.3;
  }

  .tone-success {
    border-color: color-mix(in srgb, var(--success) 32%, var(--border));
  }

  .tone-warning {
    border-color: color-mix(in srgb, var(--warning) 32%, var(--border));
  }

  .tone-danger {
    border-color: color-mix(in srgb, var(--danger) 32%, var(--border));
  }
</style>
