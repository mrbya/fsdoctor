<script lang="ts">
  import { Card, EmptyState, PageHeader, StatusBadge } from "$lib/components";
  import { projectStore } from "$lib/stores/project.svelte";
</script>

<div class="view">
  <PageHeader
    title="Dashboard"
    description="Open an existing FSDoctor project or create a new integrity record."
  />

  {#if projectStore.project === null}
    <EmptyState
      title="No project open"
      description="Create an integrity record or open an existing FSDoctor project database to begin"
    />
  {:else}
    <Card>
      <div class="project-card">
        <div>
          <StatusBadge label="Project open" tone="success" />
          <h2>{projectStore.project.name}</h2>
          <p>{projectStore.project.rootPath}</p>
        </div>
      </div>
    </Card>
  {/if}
</div>

<style>
  .view {
    display: grid;
    gap: var(--fd-space-lg);
  }

  .project-card {
    display: grid;
    gap: var(--fd-space-md);
  }

  h2,
  p {
    margin: 0;
  }

  p {
    color: var(--fd-color-text-muted);
  }
</style>
