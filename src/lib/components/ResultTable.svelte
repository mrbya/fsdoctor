<script lang="ts">
  let {
    message = "No results to display yet.",
  }: {
    message?: string;
  } = $props();
</script>

<div class="result-table-placeholder">
  <div class="eyebrow">Placeholder</div>
  <p>{message}</p>
</div>

<style>
  .result-table-placeholder {
    display: grid;
    gap: var(--space-xs);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: 0.9rem;
    color: var(--text-muted);
    background: var(--bg-soft);
  }

  .eyebrow,
  p {
    margin: 0;
  }

  .eyebrow {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }
</style>
