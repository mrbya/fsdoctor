<script lang="ts">
  import {
    Button,
    Card,
    EmptyState,
    PageHeader,
    ProgressPanel,
    StatusBadge,
  } from "$lib/components";
  import { integrityCheckStore } from "$lib/stores/integrityCheck.svelte";
  import { projectStore } from "$lib/stores/project.svelte";

  async function startCheck(): Promise<void> {
    if (projectStore.dbPath === null) {
      return;
    }

    await integrityCheckStore.start(projectStore.dbPath);
  }

  async function cancelCheck(): Promise<void> {
    await integrityCheckStore.cancel();
  }

  function statusText(status: string): string {
    if (status === "running") {
      return "Checking the current backup tree against the latest completed manifest.";
    }

    if (status === "cancelling") {
      return "Cancellation requested. FSDoctor is finishing the current safe step.";
    }

    if (status === "completed") {
      return "Integrity check completed.";
    }

    if (status === "cancelled") {
      return "Integrity check was cancelled.";
    }

    if (status === "failed") {
      return "Integrity check failed.";
    }

    return "No integrity check is running.";
  }

  function phaseText(phase: string): string {
    if (phase === "loading_manifest") {
      return "Loading manifest";
    }

    if (phase === "walking_and_checking") {
      return "Checking files";
    }

    if (phase === "recording_missing_entries") {
      return "Recording missing entries";
    }

    if (phase === "writing") {
      return "Writing results";
    }

    if (phase === "finishing") {
      return "Finishing";
    }

    return phase;
  }
</script>

<div class="view">
  <PageHeader
    title="Check backup"
    description="Verify the current backup folder against the latest completed FSDoctor manifest."
  />

  {#if projectStore.project === null}
    <Card>
      <EmptyState
        title="No project open"
        description="Create or open an FSDoctor project before running an integrity check."
      />
    </Card>
  {:else}
    <Card>
      <div class="action-card">
        <div>
          <StatusBadge label="Project open" tone="success" />
          <h2>{projectStore.project.name}</h2>
          <p>{projectStore.project.rootPath}</p>
        </div>

        <div class="actions">
          <Button
            type="button"
            disabled={integrityCheckStore.isActive ||
              projectStore.dbPath === null}
            onclick={startCheck}
          >
            Start integrity check
          </Button>

          {#if integrityCheckStore.isActive}
            <Button type="button" variant="secondary" onclick={cancelCheck}>
              Cancel
            </Button>
          {/if}
        </div>
      </div>
    </Card>
  {/if}

  {#if integrityCheckStore.status !== "idle"}
    <Card tone={integrityCheckStore.status === "failed" ? "danger" : "default"}>
      <StatusBadge
        label={integrityCheckStore.status}
        tone={integrityCheckStore.status === "failed"
          ? "danger"
          : integrityCheckStore.status === "completed"
            ? "success"
            : "info"}
      />

      <ProgressPanel
        title="Integrity check"
        description={statusText(integrityCheckStore.status)}
      />

      {#if integrityCheckStore.progress !== null}
        <section class="panel">
          <h2>{phaseText(integrityCheckStore.progress.phase)}</h2>

          {#if integrityCheckStore.progress.currentPath !== null}
            <p class="path">{integrityCheckStore.progress.currentPath}</p>
          {/if}

          <dl class="summary">
            <dt>Files seen</dt>
            <dd>{integrityCheckStore.progress.filesSeen}</dd>

            <dt>Directories seen</dt>
            <dd>{integrityCheckStore.progress.dirsSeen}</dd>

            <dt>Files hashed</dt>
            <dd>{integrityCheckStore.progress.filesHashed}</dd>

            <dt>Results written</dt>
            <dd>{integrityCheckStore.progress.resultsWritten}</dd>

            <dt>OK</dt>
            <dd>{integrityCheckStore.progress.summary.ok}</dd>

            <dt>Missing</dt>
            <dd>{integrityCheckStore.progress.summary.missing}</dd>

            <dt>New</dt>
            <dd>{integrityCheckStore.progress.summary.new}</dd>

            <dt>Hash mismatches</dt>
            <dd>{integrityCheckStore.progress.summary.hashMismatch}</dd>

            <dt>Size mismatches</dt>
            <dd>{integrityCheckStore.progress.summary.sizeMismatch}</dd>

            <dt>Type changes</dt>
            <dd>{integrityCheckStore.progress.summary.typeChanged}</dd>

            <dt>Unreadable</dt>
            <dd>{integrityCheckStore.progress.summary.unreadable}</dd>
          </dl>
        </section>
      {/if}

      {#if integrityCheckStore.report !== null}
        <section class="panel">
          <h2>Final summary</h2>

          <dl class="summary">
            <dt>Baseline manifest scan</dt>
            <dd>{integrityCheckStore.report.manifestScanId}</dd>

            <dt>OK</dt>
            <dd>{integrityCheckStore.report.summary.ok}</dd>

            <dt>Missing</dt>
            <dd>{integrityCheckStore.report.summary.missing}</dd>

            <dt>New</dt>
            <dd>{integrityCheckStore.report.summary.new}</dd>

            <dt>Hash mismatches</dt>
            <dd>{integrityCheckStore.report.summary.hashMismatch}</dd>

            <dt>Size mismatches</dt>
            <dd>{integrityCheckStore.report.summary.sizeMismatch}</dd>

            <dt>Type changes</dt>
            <dd>{integrityCheckStore.report.summary.typeChanged}</dd>

            <dt>Unreadable</dt>
            <dd>{integrityCheckStore.report.summary.unreadable}</dd>

            <dt>Changed during check</dt>
            <dd>{integrityCheckStore.report.summary.changedDuringCheck}</dd>

            <dt>Skipped</dt>
            <dd>{integrityCheckStore.report.summary.skipped}</dd>
          </dl>
        </section>
      {/if}

      {#if integrityCheckStore.error !== null}
        <section class="panel">
          <h2>Something went wrong</h2>
          <p>{integrityCheckStore.error.message}</p>

          {#if integrityCheckStore.error.details !== null}
            <details>
              <summary>Technical details</summary>
              <pre>{integrityCheckStore.error.details}</pre>
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

  .action-card {
    display: grid;
    gap: var(--space-md);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .panel {
    display: grid;
    gap: var(--space-sm);
    margin-top: var(--space-lg);
  }

  .summary {
    display: grid;
    grid-template-columns: max-content minmax(0, 1fr);
    gap: var(--space-xs) var(--space-md);
    margin: 0;
  }

  .summary dt {
    color: var(--text-muted);
  }

  .summary dd {
    margin: 0;
  }

  .path {
    overflow-wrap: anywhere;
    color: var(--text-muted);
  }

  h2,
  p {
    margin: 0;
  }

  pre {
    overflow: auto;
    border-radius: var(--radius-md);
    padding: var(--space-md);
    background: var(--bg-elevated);
  }
</style>
