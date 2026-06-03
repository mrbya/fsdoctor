<script lang="ts">
  let {
    message = "No results to display yet.",
  }: {
    message?: string;
  } = $props();
</script>

<div class="result-table-placeholder">
  {message}
</div>

<style>
  .result-table-placeholder {
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    color: var(--text-muted);
    background: var(--bg-muted);
  }
</style>
