<script lang="ts">
  import {
    Button,
    Card,
    FilePickerRow,
    PageHeader,
    StatusBadge,
    ProgressPanel,
  } from "$lib/components";
  import { projectStore } from "$lib/stores/project.svelte";
  import {
    formatBytes,
    formatCount,
    isPathInsideRoot,
  } from "$lib/utils/helpers";
  import { manifestGenerationStore } from "$lib/stores/manifestGeneration.svelte";

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

  async function startManifest(): Promise<void> {
    if (projectStore.dbPath === null) {
      return;
    }

    await manifestGenerationStore.start(projectStore.dbPath);
  }

  async function cancelManifest(): Promise<void> {
    await manifestGenerationStore.cancel();
  }

  function manifestGenerationStatusText(status: string): string {
    if (status === "running") {
      return "Scanning files and writing the manifest. This may take a while.";
    }

    if (status === "cancelling") {
      return "Cancellation requested. FSDoctor is finishing the current safe step.";
    }

    if (status === "completed") {
      return "Manifest generation completed successfully.";
    }

    if (status === "cancelled") {
      return "Manifest generation was cancelled.";
    }

    if (status === "failed") {
      return "Manifest generation failed.";
    }

    return "No manifest generation job is running.";
  }

  function manifestPhaseText(phase: string): string {
    if (phase === "walking_and_hashing") {
      return "Scanning and hashing files";
    }

    if (phase === "writing") {
      return "Writing manifest entries";
    }

    if (phase === "finishing") {
      return "Finishing up";
    }

    return phase;
  }

  function statusTone(
    status: string,
  ): "neutral" | "success" | "warning" | "danger" | "info" {
    if (status === "completed") {
      return "success";
    }

    if (status === "cancelled") {
      return "warning";
    }

    if (status === "failed") {
      return "danger";
    }

    return "info";
  }
</script>

<div class="view">
  <PageHeader
    title="Create integrity record"
    description="Create or open an FSDoctor project, then scan the backup tree into a portable integrity record."
  />

  <div class="grid">
    <Card>
      <form class="form" onsubmit={create}>
        <div class="section-heading">
          <h2>Create project</h2>
          <p>
            Start a new integrity database for the backup root you want to
            track.
          </p>
        </div>

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
        <div class="section-heading">
          <h2>Open existing project</h2>
          <p>Re-open a previously created `.fsdoctor.sqlite` database.</p>
        </div>

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
      <div class="stack-sm">
        <StatusBadge label="Project action failed" tone="danger" />
        <h2>Something went wrong</h2>
        <p>{projectStore.error.message}</p>
      </div>

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
      <div class="stack-sm">
        <StatusBadge label="Project ready" tone="success" />
        <h2>{projectStore.project.name}</h2>
        <p>{projectStore.project.rootPath}</p>
      </div>
    </Card>

    <Card>
      <div class="job-card">
        <div class="section-heading">
          <h2>Generate manifest</h2>
          <p>
            Scan the selected backup root, hash regular files, and persist the
            manifest into the FSDoctor project database.
          </p>
        </div>

        <div class="actions">
          <Button
            type="button"
            disabled={manifestGenerationStore.isActive ||
              projectStore.dbPath === null}
            onclick={startManifest}
          >
            Generate manifest
          </Button>

          {#if manifestGenerationStore.isActive}
            <Button type="button" variant="secondary" onclick={cancelManifest}>
              Cancel scan
            </Button>
          {/if}
        </div>

        {#if !manifestGenerationStore.isActive && manifestGenerationStore.status === "idle"}
          <p class="supporting-text">
            Progress appears here once the scan starts. FSDoctor shows counters,
            not a fake percentage.
          </p>
        {/if}
      </div>
    </Card>
  {/if}

  {#if manifestGenerationStore.status !== "idle"}
    <Card
      tone={manifestGenerationStore.status === "failed"
        ? "danger"
        : manifestGenerationStore.status === "completed"
          ? "success"
          : "default"}
    >
      <div class="stack-lg" aria-live="polite">
        <StatusBadge
          label={manifestGenerationStore.status}
          tone={statusTone(manifestGenerationStore.status)}
        />

        <ProgressPanel
          eyebrow="Manifest generation"
          title={manifestGenerationStore.progress === null
            ? "Preparing scan"
            : manifestPhaseText(manifestGenerationStore.progress.phase)}
          description={manifestGenerationStatusText(
            manifestGenerationStore.status,
          )}
        />

        {#if manifestGenerationStore.progress !== null}
          <section class="panel">
            <div class="section-heading compact">
              <h2>Live progress</h2>
              <p>
                These counters update as FSDoctor discovers, hashes, and writes
                entries.
              </p>
            </div>

            {#if manifestGenerationStore.progress.currentPath !== null}
              <div class="path-block">
                <span class="path-label">Current path</span>
                <code>{manifestGenerationStore.progress.currentPath}</code>
              </div>
            {/if}

            <dl class="summary-grid">
              <div>
                <dt>Files seen</dt>
                <dd>
                  {formatCount(manifestGenerationStore.progress.filesSeen)}
                </dd>
              </div>
              <div>
                <dt>Directories seen</dt>
                <dd>
                  {formatCount(manifestGenerationStore.progress.dirsSeen)}
                </dd>
              </div>
              <div>
                <dt>Data discovered</dt>
                <dd>
                  {formatBytes(manifestGenerationStore.progress.bytesSeen)}
                </dd>
              </div>
              <div>
                <dt>Files hashed</dt>
                <dd>
                  {formatCount(manifestGenerationStore.progress.filesHashed)}
                </dd>
              </div>
              <div>
                <dt>Unreadable entries</dt>
                <dd>
                  {formatCount(
                    manifestGenerationStore.progress.unreadableEntries,
                  )}
                </dd>
              </div>
              <div>
                <dt>Changed during scan</dt>
                <dd>
                  {formatCount(
                    manifestGenerationStore.progress.changedDuringScan,
                  )}
                </dd>
              </div>
              <div>
                <dt>Entries written</dt>
                <dd>
                  {formatCount(manifestGenerationStore.progress.resultsWritten)}
                </dd>
              </div>
            </dl>
          </section>
        {/if}

        {#if manifestGenerationStore.report !== null}
          <section class="panel final-summary">
            <div class="section-heading compact">
              <h2>Final summary</h2>
              <p>
                The latest completed manifest snapshot is stored in the project
                database.
              </p>
            </div>

            <dl class="summary-grid">
              <div>
                <dt>Scan ID</dt>
                <dd>{formatCount(manifestGenerationStore.report.scanId)}</dd>
              </div>
              <div>
                <dt>Files seen</dt>
                <dd>
                  {formatCount(manifestGenerationStore.report.totalFiles)}
                </dd>
              </div>
              <div>
                <dt>Directories seen</dt>
                <dd>{formatCount(manifestGenerationStore.report.totalDirs)}</dd>
              </div>
              <div>
                <dt>Files hashed</dt>
                <dd>
                  {formatCount(manifestGenerationStore.report.hashedFiles)}
                </dd>
              </div>
              <div>
                <dt>Total data</dt>
                <dd>
                  {formatBytes(manifestGenerationStore.report.totalBytes)}
                </dd>
              </div>
              <div>
                <dt>Unreadable entries</dt>
                <dd>
                  {formatCount(
                    manifestGenerationStore.report.unreadableEntries,
                  )}
                </dd>
              </div>
              <div>
                <dt>Changed during scan</dt>
                <dd>
                  {formatCount(
                    manifestGenerationStore.report.changedDuringScan,
                  )}
                </dd>
              </div>
              <div>
                <dt>Skipped special entries</dt>
                <dd>
                  {formatCount(
                    manifestGenerationStore.report.totalSymlinks +
                      manifestGenerationStore.report.totalOther,
                  )}
                </dd>
              </div>
            </dl>
          </section>
        {/if}
      </div>

      {#if manifestGenerationStore.error !== null}
        <section class="panel error-panel">
          <h2>Something went wrong</h2>
          <p>{manifestGenerationStore.error.message}</p>

          {#if manifestGenerationStore.error.details !== null}
            <details>
              <summary>Technical details</summary>
              <pre>{manifestGenerationStore.error.details}</pre>
            </details>
          {/if}
        </section>
      {/if}
    </Card>
  {/if}
</div>

<style>
  .view {
    display: grid;
    gap: var(--space-lg);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--space-lg);
  }

  .form {
    display: grid;
    gap: var(--space-md);
  }

  label {
    display: grid;
    gap: var(--space-xs);
    color: var(--text-muted);
  }

  input {
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 0.7rem;
    color: var(--text);
    background: var(--bg-elevated);
  }

  h2,
  p {
    margin: 0;
  }

  p {
    color: var(--text-muted);
  }

  .warning {
    display: grid;
    gap: var(--space-xs);
    border: 1px solid color-mix(in srgb, var(--warning), transparent 50%);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    background: color-mix(in srgb, var(--warning), transparent 92%);
  }

  .job-card,
  .stack-lg,
  .panel,
  .stack-sm {
    display: grid;
    gap: var(--space-md);
  }

  .section-heading {
    display: grid;
    gap: var(--space-xs);
  }

  .section-heading.compact {
    gap: var(--space-2xs);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .supporting-text {
    font-size: var(--font-size-sm);
  }

  .path-block {
    display: grid;
    gap: var(--space-2xs);
    padding: var(--space-md);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    background: var(--bg-muted);
  }

  .path-label,
  .summary-grid dt {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
  }

  .path-block code {
    overflow-wrap: anywhere;
    color: var(--text);
  }

  .summary-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr));
    gap: var(--space-sm);
  }

  .summary-grid div {
    display: grid;
    gap: var(--space-2xs);
    padding: var(--space-md);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    background: color-mix(in srgb, var(--bg-raised), transparent 12%);
  }

  .summary-grid dd {
    margin: 0;
    color: var(--text);
    font-size: var(--font-size-lg);
  }

  .final-summary {
    border-top: 1px solid var(--border);
    padding-top: var(--space-md);
  }

  .error-panel {
    border-top: 1px solid color-mix(in srgb, var(--danger), transparent 70%);
    padding-top: var(--space-md);
  }

  pre {
    overflow: auto;
    border-radius: var(--radius-md);
    padding: var(--space-md);
    background: var(--bg-elevated);
  }

  @media (max-width: 64rem) {
    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
