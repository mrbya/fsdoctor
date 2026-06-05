<script lang="ts">
  import { Card, EmptyState, PageHeader, StatusBadge } from "$lib/components";
  import { projectStore } from "$lib/stores/project.svelte";
</script>

<div class="view">
  <PageHeader
    title="Dashboard"
    description="A compact overview of the currently opened project and the two core FSDoctor workflows."
  />

  {#if projectStore.project === null}
    <Card title="Overview">
      <EmptyState
        title="No project open"
        description="Create an integrity record or open an existing FSDoctor project database to begin."
      />
    </Card>
  {:else}
    <div class="dashboard-grid">
      <Card title="Current project">
        <div class="project-card">
          <StatusBadge label="Project open" tone="success" />
          <h2>{projectStore.project.name}</h2>
          <dl class="project-meta">
            <div>
              <dt>Backup root</dt>
              <dd>{projectStore.project.rootPath}</dd>
            </div>
            <div>
              <dt>Database path</dt>
              <dd>{projectStore.dbPath}</dd>
            </div>
          </dl>
        </div>
      </Card>

      <Card title="Workflows">
        <div class="workflow-list">
          <div>
            <h2>Create manifest</h2>
            <p>Scan the backup root and save a completed baseline snapshot.</p>
          </div>
          <div>
            <h2>Check backup</h2>
            <p>
              Compare the current tree against the latest completed manifest.
            </p>
          </div>
        </div>
      </Card>
    </div>
  {/if}
</div>

<style>
  .view {
    display: grid;
    gap: var(--space-lg);
  }

  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--space-lg);
  }

  .project-card,
  .workflow-list,
  .workflow-list div {
    display: grid;
    gap: var(--space-sm);
  }

  h2,
  p {
    margin: 0;
  }

  p {
    color: var(--text-muted);
  }

  .project-meta {
    display: grid;
    gap: var(--space-sm);
  }

  .project-meta div {
    display: grid;
    gap: var(--space-2xs);
  }

  .project-meta dt {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
  }

  .project-meta dd {
    margin: 0;
    color: var(--text);
    overflow-wrap: anywhere;
  }

  @media (max-width: 64rem) {
    .dashboard-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
