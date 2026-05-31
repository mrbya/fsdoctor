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
    border: 1px solid var(--fd-color-border);
    border-radius: var(--fd-radius-lg);
    padding: var(--fd-space-lg);
    color: var(--fd-color-text-muted);
    background: var(--fd-color-bg-muted);
  }
</style>
