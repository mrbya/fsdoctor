<script lang="ts">
  import {
    Button,
    Card,
    FilePickerRow,
    MetricGrid,
    PageHeader,
    ProgressPanel,
    StatusBadge,
  } from "$lib/components";
  import { manifestGenerationStore } from "$lib/stores/manifestGeneration.svelte";
  import { projectStore } from "$lib/stores/project.svelte";
  import {
    formatBytes,
    formatCount,
    isPathInsideRoot,
  } from "$lib/utils/helpers";

  let projectName = $state("My Backup");
  let rootPath = $state("");
  let dbPath = $state("");

  const showDbLocationWarning = $derived(isPathInsideRoot(dbPath, rootPath));

  const liveMetrics = $derived.by(() => {
    const progress = manifestGenerationStore.progress;

    if (progress === null) {
      return [];
    }

    return [
      { label: "Files seen", value: formatCount(progress.filesSeen) },
      { label: "Directories seen", value: formatCount(progress.dirsSeen) },
      { label: "Data discovered", value: formatBytes(progress.bytesSeen) },
      { label: "Files hashed", value: formatCount(progress.filesHashed) },
      {
        label: "Unreadable entries",
        value: formatCount(progress.unreadableEntries),
        tone:
          progress.unreadableEntries > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Changed during scan",
        value: formatCount(progress.changedDuringScan),
        tone:
          progress.changedDuringScan > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Entries written",
        value: formatCount(progress.resultsWritten),
      },
    ];
  });

  const reportMetrics = $derived.by(() => {
    const report = manifestGenerationStore.report;

    if (report === null) {
      return [];
    }

    return [
      { label: "Scan ID", value: formatCount(report.scanId) },
      { label: "Files seen", value: formatCount(report.totalFiles) },
      { label: "Directories seen", value: formatCount(report.totalDirs) },
      { label: "Files hashed", value: formatCount(report.hashedFiles) },
      { label: "Total data", value: formatBytes(report.totalBytes) },
      {
        label: "Unreadable entries",
        value: formatCount(report.unreadableEntries),
        tone:
          report.unreadableEntries > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Changed during scan",
        value: formatCount(report.changedDuringScan),
        tone:
          report.changedDuringScan > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Skipped special entries",
        value: formatCount(report.totalSymlinks + report.totalOther),
      },
    ];
  });

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
      return "Scanning files and writing the manifest. Counters update as work completes.";
    }

    if (status === "cancelling") {
      return "Cancellation was requested. FSDoctor will stop after the current safe step.";
    }

    if (status === "completed") {
      return "Manifest generation finished successfully and the snapshot is saved in the project database.";
    }

    if (status === "cancelled") {
      return "Manifest generation stopped before completion.";
    }

    if (status === "failed") {
      return "Manifest generation failed before a completed snapshot was recorded.";
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
      return "Finishing";
    }

    return phase;
  }

  function statusTone(
    status: string,
  ): "neutral" | "success" | "warning" | "danger" | "info" {
    if (status === "completed") {
      return "success";
    }

    if (status === "cancelled" || status === "cancelling") {
      return "warning";
    }

    if (status === "failed") {
      return "danger";
    }

    return "info";
  }

  function statusCardTone(
    status: string,
  ): "default" | "success" | "warning" | "danger" {
    if (status === "completed") {
      return "success";
    }

    if (status === "cancelled" || status === "cancelling") {
      return "warning";
    }

    if (status === "failed") {
      return "danger";
    }

    return "default";
  }
</script>

<div class="view">
  <PageHeader
    title="Create integrity record"
    description="Create or open an FSDoctor project, then scan the backup tree into a portable integrity record."
  />

  <div class="setup-grid">
    <Card title="Create project">
      <form class="form" onsubmit={create}>
        <div class="section-heading">
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
          <div class="notice warning">
            <StatusBadge label="Safety warning" tone="warning" />
            <p>
              The database appears to be inside the backup root. Save it
              elsewhere to avoid losing both the source data and its integrity
              record at the same time.
            </p>
          </div>
        {/if}

        <div class="actions">
          <Button
            type="submit"
            disabled={projectStore.busy}
            loading={projectStore.busy}
          >
            Create project
          </Button>
        </div>
      </form>
    </Card>

    <Card title="Open existing project">
      <form class="form" onsubmit={open}>
        <div class="section-heading">
          <p>Re-open a previously created `.fsdoctor.sqlite` database.</p>
        </div>

        <FilePickerRow
          label="FSDoctor database path"
          bind:value={dbPath}
          placeholder="C:\\Users\\User\\Documents\\OldShare.fsdoctor.sqlite"
          buttonLabel="Browse"
        />

        <div class="actions">
          <Button
            type="submit"
            variant="secondary"
            disabled={projectStore.busy}
            loading={projectStore.busy}
          >
            Open project
          </Button>
        </div>
      </form>
    </Card>
  </div>

  {#if projectStore.error !== null}
    <Card tone="danger" title="Project action failed">
      <div class="stack" aria-live="polite">
        <StatusBadge label="Project action failed" tone="danger" />
        <p>{projectStore.error.message}</p>

        {#if projectStore.error.details !== null}
          <details>
            <summary>Technical details</summary>
            <pre>{projectStore.error.details}</pre>
          </details>
        {/if}
      </div>
    </Card>
  {/if}

  {#if projectStore.project !== null}
    <div class="workflow-stack">
      <Card title="Current project">
        <div class="project-summary">
          <div class="project-heading">
            <StatusBadge label="Project ready" tone="success" />
            <h2>{projectStore.project.name}</h2>
          </div>

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

      <Card title="Manifest generation">
        <div class="stack">
          <div class="section-heading">
            <p>
              Scan the selected backup root, hash regular files, and persist a
              completed manifest snapshot into the current project.
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
              <Button
                type="button"
                variant="secondary"
                loading={manifestGenerationStore.status === "cancelling"}
                onclick={cancelManifest}
              >
                Cancel scan
              </Button>
            {/if}
          </div>

          {#if !manifestGenerationStore.isActive && manifestGenerationStore.status === "idle"}
            <p class="supporting-text">
              Progress appears here once the scan starts. FSDoctor shows phases
              and real counters, not a fake percentage.
            </p>
          {/if}
        </div>
      </Card>
    </div>
  {/if}

  {#if manifestGenerationStore.status !== "idle"}
    <Card
      title="Manifest job"
      tone={statusCardTone(manifestGenerationStore.status)}
    >
      <div class="stack" aria-live="polite">
        <StatusBadge
          label={manifestGenerationStore.status}
          tone={statusTone(manifestGenerationStore.status)}
        />

        <ProgressPanel
          title={manifestGenerationStore.progress === null
            ? "Preparing scan"
            : manifestPhaseText(manifestGenerationStore.progress.phase)}
          description={manifestGenerationStatusText(
            manifestGenerationStore.status,
          )}
        />

        {#if manifestGenerationStore.progress !== null}
          <section class="detail-section">
            <div class="section-heading compact">
              <h2>Live progress</h2>
              <p>
                These counters update while files are discovered, hashed, and
                written.
              </p>
            </div>

            {#if manifestGenerationStore.progress.currentPath !== null}
              <div class="path-block">
                <span class="path-label">Current path</span>
                <code>{manifestGenerationStore.progress.currentPath}</code>
              </div>
            {/if}

            <MetricGrid items={liveMetrics} />
          </section>
        {/if}

        {#if manifestGenerationStore.report !== null}
          <section class="detail-section final-summary">
            <div class="section-heading compact">
              <h2>Final summary</h2>
              <p>The latest completed manifest snapshot is summarized below.</p>
            </div>

            <MetricGrid items={reportMetrics} />
          </section>
        {/if}

        {#if manifestGenerationStore.error !== null}
          <section class="detail-section error-section">
            <StatusBadge label="Manifest generation failed" tone="danger" />
            <p>{manifestGenerationStore.error.message}</p>

            {#if manifestGenerationStore.error.details !== null}
              <details>
                <summary>Technical details</summary>
                <pre>{manifestGenerationStore.error.details}</pre>
              </details>
            {/if}
          </section>
        {/if}
      </div>
    </Card>
  {/if}
</div>

<style>
  .view {
    display: grid;
    gap: var(--space-lg);
  }

  .setup-grid,
  .workflow-stack {
    display: grid;
    gap: var(--space-lg);
  }

  .setup-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .form,
  .stack,
  .project-summary,
  .project-heading,
  .section-heading,
  .detail-section,
  .notice {
    display: grid;
    gap: var(--space-sm);
  }

  .section-heading.compact {
    gap: var(--space-2xs);
  }

  h2,
  p {
    margin: 0;
  }

  h2 {
    font-size: var(--font-size-lg);
    line-height: 1.35;
  }

  p {
    color: var(--text-muted);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .supporting-text {
    font-size: var(--font-size-sm);
  }

  .notice {
    padding: 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-soft);
  }

  .notice.warning {
    border-color: color-mix(in srgb, var(--warning) 40%, var(--border));
    background: var(--bg-warning-soft);
  }

  .project-meta {
    display: grid;
    gap: var(--space-sm);
  }

  .project-meta div {
    display: grid;
    gap: var(--space-2xs);
  }

  .project-meta dt,
  .path-label {
    color: var(--text-muted);
    font-size: var(--font-size-sm);
  }

  .project-meta dd {
    margin: 0;
    color: var(--text);
    overflow-wrap: anywhere;
  }

  .path-block {
    display: grid;
    gap: var(--space-2xs);
    padding: 0.75rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--bg-soft);
  }

  .path-block code {
    color: var(--text);
    overflow-wrap: anywhere;
  }

  .final-summary,
  .error-section {
    padding-top: var(--space-md);
    border-top: 1px solid var(--border);
  }

  .error-section {
    border-top-color: color-mix(in srgb, var(--danger) 24%, var(--border));
  }

  @media (max-width: 64rem) {
    .setup-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
