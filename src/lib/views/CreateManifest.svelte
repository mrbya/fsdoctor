<script lang="ts">
  import {
    Button,
    Card,
    FilePickerRow,
    PageHeader,
    StatusBadge,
  } from "$lib/components";
  import { projectStore } from "$lib/stores/project.svelte";
  import { isPathInsideRoot } from "$lib/utils/helpers";

  let projectName = $state("My Backup");
  let rootPath = $state("");
  let dbPath = $state("");

  let showDbLocationWarning = $derived(isPathInsideRoot(dbPath, rootPath));

  async function create(event: Event): Promise<void> {
    event.preventDefault();

    await projectStore.create({
      name: projectName,
      rootPath,
      dbPath,
    });
  }

  async function open(event: Event): Promise<void> {
    event.preventDefault();

    await projectStore.open({ dbPath });
  }
</script>

<div class="view">
  <PageHeader
    title="Create integrity record"
    description="Create an FSDoctor project database for a backup folder. File scanning is implemented in later phases."
  />

  <div class="grid">
    <Card>
      <form class="form" onsubmit={create}>
        <h2>Create project</h2>

        <label>
          <span>Project name</span>
          <input bind:value={projectName} placeholder="My Backup" />
        </label>

        <FilePickerRow
          label="Backup root path"
          bind:value={rootPath}
          placeholder="D:\\Backups\\OldShare"
          buttonLabel="Browse"
        />

        <FilePickerRow
          label="FSDoctor database path"
          bind:value={dbPath}
          placeholder="C:\\Users\\User\\Documents\\OldShare.fsdoctor.sqlite"
          buttonLabel="Browse"
        />

        {#if showDbLocationWarning}
          <div class="warning">
            <StatusBadge label="Safety warning" tone="warning" />
            <p>
              The integrity database appears to be inside the backup folder. For
              better protection, save it somewhere else.
            </p>
          </div>
        {/if}

        <Button type="submit" disabled={projectStore.busy}>
          Create project
        </Button>
      </form>
    </Card>

    <Card>
      <form class="form" onsubmit={open}>
        <h2>Open existing project</h2>

        <FilePickerRow
          label="FSDoctor database path"
          bind:value={dbPath}
          placeholder="C:\\Users\\User\\Documents\\OldShare.fsdoctor.sqlite"
          buttonLabel="Browse"
        />

        <Button type="submit" variant="secondary" disabled={projectStore.busy}>
          Open project
        </Button>
      </form>
    </Card>
  </div>

  {#if projectStore.error !== null}
    <Card tone="danger">
      <StatusBadge label={projectStore.error.kind} tone="danger" />
      <h2>Something went wrong</h2>
      <p>{projectStore.error.message}</p>

      {#if projectStore.error.details !== null}
        <details>
          <summary>Technical details</summary>
          <pre>{projectStore.error.details}</pre>
        </details>
      {/if}
    </Card>
  {/if}

  {#if projectStore.project !== null}
    <Card tone="success">
      <StatusBadge label="Project ready" tone="success" />
      <h2>{projectStore.project.name}</h2>
      <p>{projectStore.project.rootPath}</p>
    </Card>
  {/if}
</div>

<style>
  .view {
    display: grid;
    gap: var(--fd-space-lg);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--fd-space-lg);
  }

  .form {
    display: grid;
    gap: var(--fd-space-md);
  }

  label {
    display: grid;
    gap: var(--fd-space-xs);
    color: var(--fd-color-text-muted);
  }

  input {
    border: 1px solid var(--fd-color-border);
    border-radius: var(--fd-radius-md);
    padding: 0.7rem;
    color: var(--fd-color-text);
    background: var(--fd-color-bg-elevated);
  }

  h2,
  p {
    margin: 0;
  }

  p {
    color: var(--fd-color-text-muted);
  }

  .warning {
    display: grid;
    gap: var(--fd-space-xs);
    border: 1px solid
      color-mix(in srgb, var(--fd-color-warning), transparent 50%);
    border-radius: var(--fd-radius-md);
    padding: var(--fd-space-md);
    background: color-mix(in srgb, var(--fd-color-warning), transparent 92%);
  }

  pre {
    overflow: auto;
    border-radius: var(--fd-radius-md);
    padding: var(--fd-space-md);
    background: var(--fd-color-bg-elevated);
  }

  @media (max-width: 64rem) {
    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
