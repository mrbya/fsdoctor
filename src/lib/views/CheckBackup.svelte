<script lang="ts">
  import {
    Button,
    Card,
    EmptyState,
    MetricGrid,
    PageHeader,
    ProgressPanel,
    StatusBadge,
  } from "$lib/components";
  import { integrityCheckStore } from "$lib/stores/integrityCheck.svelte";
  import { projectStore } from "$lib/stores/project.svelte";
  import { formatBytes, formatCount } from "$lib/utils/helpers";

  const missingManifest = $derived(
    integrityCheckStore.error?.kind === "no_completed_manifest",
  );

  const liveMetrics = $derived.by(() => {
    const progress = integrityCheckStore.progress;

    if (progress === null) {
      return [];
    }

    return [
      { label: "Files seen", value: formatCount(progress.filesSeen) },
      { label: "Directories seen", value: formatCount(progress.dirsSeen) },
      { label: "Data checked", value: formatBytes(progress.bytesSeen) },
      { label: "Files hashed", value: formatCount(progress.filesHashed) },
      { label: "Results written", value: formatCount(progress.resultsWritten) },
      {
        label: "OK",
        value: formatCount(progress.summary.ok),
        tone: "success" as const,
      },
      {
        label: "Missing",
        value: formatCount(progress.summary.missing),
        tone:
          progress.summary.missing > 0
            ? ("danger" as const)
            : ("default" as const),
      },
      {
        label: "New",
        value: formatCount(progress.summary.new),
        tone:
          progress.summary.new > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Hash mismatch",
        value: formatCount(progress.summary.hashMismatch),
        tone:
          progress.summary.hashMismatch > 0
            ? ("danger" as const)
            : ("default" as const),
      },
      {
        label: "Size mismatch",
        value: formatCount(progress.summary.sizeMismatch),
        tone:
          progress.summary.sizeMismatch > 0
            ? ("danger" as const)
            : ("default" as const),
      },
      {
        label: "Type changed",
        value: formatCount(progress.summary.typeChanged),
        tone:
          progress.summary.typeChanged > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Unreadable",
        value: formatCount(progress.summary.unreadable),
        tone:
          progress.summary.unreadable > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Changed during check",
        value: formatCount(progress.summary.changedDuringCheck),
        tone:
          progress.summary.changedDuringCheck > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Skipped",
        value: formatCount(progress.summary.skipped),
        tone:
          progress.summary.skipped > 0
            ? ("warning" as const)
            : ("default" as const),
      },
    ];
  });

  const reportMetrics = $derived.by(() => {
    const report = integrityCheckStore.report;

    if (report === null) {
      return [];
    }

    return [
      { label: "Check scan ID", value: formatCount(report.scanId) },
      {
        label: "Baseline manifest scan",
        value: formatCount(report.manifestScanId),
      },
      {
        label: "OK",
        value: formatCount(report.summary.ok),
        tone: "success" as const,
      },
      {
        label: "Missing",
        value: formatCount(report.summary.missing),
        tone:
          report.summary.missing > 0
            ? ("danger" as const)
            : ("default" as const),
      },
      {
        label: "New",
        value: formatCount(report.summary.new),
        tone:
          report.summary.new > 0 ? ("warning" as const) : ("default" as const),
      },
      {
        label: "Hash mismatch",
        value: formatCount(report.summary.hashMismatch),
        tone:
          report.summary.hashMismatch > 0
            ? ("danger" as const)
            : ("default" as const),
      },
      {
        label: "Size mismatch",
        value: formatCount(report.summary.sizeMismatch),
        tone:
          report.summary.sizeMismatch > 0
            ? ("danger" as const)
            : ("default" as const),
      },
      {
        label: "Type changed",
        value: formatCount(report.summary.typeChanged),
        tone:
          report.summary.typeChanged > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Unreadable",
        value: formatCount(report.summary.unreadable),
        tone:
          report.summary.unreadable > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Changed during check",
        value: formatCount(report.summary.changedDuringCheck),
        tone:
          report.summary.changedDuringCheck > 0
            ? ("warning" as const)
            : ("default" as const),
      },
      {
        label: "Skipped",
        value: formatCount(report.summary.skipped),
        tone:
          report.summary.skipped > 0
            ? ("warning" as const)
            : ("default" as const),
      },
    ];
  });

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
      return "Cancellation was requested. FSDoctor will stop after the current safe step.";
    }

    if (status === "completed") {
      return "Integrity check completed and the final report is available below.";
    }

    if (status === "cancelled") {
      return "Integrity check stopped before completion.";
    }

    if (status === "failed") {
      return "Integrity check failed before a final report could be written.";
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
    title="Check backup"
    description="Verify the current backup folder against the latest completed FSDoctor manifest."
  />

  {#if projectStore.project === null}
    <Card title="Project required">
      <EmptyState
        title="No project open"
        description="Create or open an FSDoctor project before running an integrity check."
      />
    </Card>
  {:else}
    <div class="workflow-stack">
      <Card title="Current project">
        <div class="project-summary">
          <div class="project-heading">
            <StatusBadge label="Project open" tone="success" />
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

      <Card title="Integrity check">
        <div class="stack">
          <div class="section-heading">
            <p>
              Compare the current backup tree against the latest completed
              integrity record and summarize anything missing, new, changed,
              unreadable, or skipped.
            </p>
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
              <Button
                type="button"
                variant="secondary"
                loading={integrityCheckStore.status === "cancelling"}
                onclick={cancelCheck}
              >
                Cancel check
              </Button>
            {/if}
          </div>

          {#if integrityCheckStore.status === "idle"}
            <p class="supporting-text">
              Progress appears here once the check starts. FSDoctor shows actual
              phases and counters while the scan is still running.
            </p>
          {/if}
        </div>
      </Card>
    </div>
  {/if}

  {#if integrityCheckStore.status !== "idle"}
    <Card
      title="Integrity job"
      tone={statusCardTone(integrityCheckStore.status)}
    >
      <div class="stack" aria-live="polite">
        <StatusBadge
          label={integrityCheckStore.status}
          tone={statusTone(integrityCheckStore.status)}
        />

        <ProgressPanel
          title={integrityCheckStore.progress === null
            ? "Preparing check"
            : phaseText(integrityCheckStore.progress.phase)}
          description={statusText(integrityCheckStore.status)}
        />

        {#if integrityCheckStore.progress !== null}
          <section class="detail-section">
            <div class="section-heading compact">
              <h2>Live progress</h2>
              <p>
                The current counters remain visible while the final results are
                being written.
              </p>
            </div>

            {#if integrityCheckStore.progress.currentPath !== null}
              <div class="path-block">
                <span class="path-label">Current path</span>
                <code>{integrityCheckStore.progress.currentPath}</code>
              </div>
            {/if}

            <MetricGrid items={liveMetrics} />
          </section>
        {/if}

        {#if integrityCheckStore.report !== null}
          <section class="detail-section final-summary">
            <div class="section-heading compact">
              <h2>Final summary</h2>
              <p>The latest finished check is summarized below.</p>
            </div>

            <MetricGrid items={reportMetrics} />
          </section>
        {/if}

        {#if integrityCheckStore.error !== null}
          <section class="detail-section error-section">
            <StatusBadge
              label={missingManifest ? "Manifest required" : "Check failed"}
              tone={missingManifest ? "warning" : "danger"}
            />
            <p>{integrityCheckStore.error.message}</p>

            {#if integrityCheckStore.error.details !== null}
              <details>
                <summary>Technical details</summary>
                <pre>{integrityCheckStore.error.details}</pre>
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

  .workflow-stack,
  .stack,
  .project-summary,
  .project-heading,
  .section-heading,
  .detail-section {
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
</style>
