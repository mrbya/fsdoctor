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
  import { formatBytes, formatCount } from "$lib/utils/helpers";

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

  const missingManifest = $derived(
    integrityCheckStore.error?.kind === "no_completed_manifest",
  );
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
        <div class="section-heading compact">
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
              Cancel check
            </Button>
          {/if}
        </div>

        {#if integrityCheckStore.status === "idle"}
          <p class="supporting-text">
            Run a check against the latest completed integrity record. FSDoctor
            compares the current backup tree and reports anything missing, new,
            changed, unreadable, or skipped.
          </p>
        {/if}
      </div>
    </Card>
  {/if}

  {#if integrityCheckStore.status !== "idle"}
    <Card tone={integrityCheckStore.status === "failed" ? "danger" : "default"}>
      <div class="stack-lg" aria-live="polite">
        <StatusBadge
          label={integrityCheckStore.status}
          tone={statusTone(integrityCheckStore.status)}
        />

        <ProgressPanel
          eyebrow="Integrity check"
          title={integrityCheckStore.progress === null
            ? "Preparing check"
            : phaseText(integrityCheckStore.progress.phase)}
          description={statusText(integrityCheckStore.status)}
        />

        {#if integrityCheckStore.progress !== null}
          <section class="panel">
            <div class="section-heading compact">
              <h2>Live progress</h2>
              <p>
                The current counters stay visible while results are being
                written.
              </p>
            </div>

            {#if integrityCheckStore.progress.currentPath !== null}
              <div class="path-block">
                <span class="path-label">Current path</span>
                <code>{integrityCheckStore.progress.currentPath}</code>
              </div>
            {/if}

            <dl class="summary-grid">
              <div>
                <dt>Files seen</dt>
                <dd>{formatCount(integrityCheckStore.progress.filesSeen)}</dd>
              </div>
              <div>
                <dt>Directories seen</dt>
                <dd>{formatCount(integrityCheckStore.progress.dirsSeen)}</dd>
              </div>
              <div>
                <dt>Data checked</dt>
                <dd>{formatBytes(integrityCheckStore.progress.bytesSeen)}</dd>
              </div>
              <div>
                <dt>Files hashed</dt>
                <dd>{formatCount(integrityCheckStore.progress.filesHashed)}</dd>
              </div>
              <div>
                <dt>Results written</dt>
                <dd>
                  {formatCount(integrityCheckStore.progress.resultsWritten)}
                </dd>
              </div>
              <div>
                <dt>OK</dt>
                <dd>{formatCount(integrityCheckStore.progress.summary.ok)}</dd>
              </div>
              <div>
                <dt>Missing</dt>
                <dd>
                  {formatCount(integrityCheckStore.progress.summary.missing)}
                </dd>
              </div>
              <div>
                <dt>New</dt>
                <dd>{formatCount(integrityCheckStore.progress.summary.new)}</dd>
              </div>
              <div>
                <dt>Hash mismatch</dt>
                <dd>
                  {formatCount(
                    integrityCheckStore.progress.summary.hashMismatch,
                  )}
                </dd>
              </div>
              <div>
                <dt>Size mismatch</dt>
                <dd>
                  {formatCount(
                    integrityCheckStore.progress.summary.sizeMismatch,
                  )}
                </dd>
              </div>
              <div>
                <dt>Type changed</dt>
                <dd>
                  {formatCount(
                    integrityCheckStore.progress.summary.typeChanged,
                  )}
                </dd>
              </div>
              <div>
                <dt>Unreadable</dt>
                <dd>
                  {formatCount(integrityCheckStore.progress.summary.unreadable)}
                </dd>
              </div>
              <div>
                <dt>Changed during check</dt>
                <dd>
                  {formatCount(
                    integrityCheckStore.progress.summary.changedDuringCheck,
                  )}
                </dd>
              </div>
              <div>
                <dt>Skipped</dt>
                <dd>
                  {formatCount(integrityCheckStore.progress.summary.skipped)}
                </dd>
              </div>
            </dl>
          </section>
        {/if}

        {#if integrityCheckStore.report !== null}
          <section class="panel final-summary">
            <div class="section-heading compact">
              <h2>Final summary</h2>
              <p>The latest finished check is summarized below.</p>
            </div>

            <dl class="summary-grid">
              <div>
                <dt>Check scan ID</dt>
                <dd>{formatCount(integrityCheckStore.report.scanId)}</dd>
              </div>
              <div>
                <dt>Baseline manifest scan</dt>
                <dd>
                  {formatCount(integrityCheckStore.report.manifestScanId)}
                </dd>
              </div>
              <div>
                <dt>OK</dt>
                <dd>{formatCount(integrityCheckStore.report.summary.ok)}</dd>
              </div>
              <div>
                <dt>Missing</dt>
                <dd>
                  {formatCount(integrityCheckStore.report.summary.missing)}
                </dd>
              </div>
              <div>
                <dt>New</dt>
                <dd>{formatCount(integrityCheckStore.report.summary.new)}</dd>
              </div>
              <div>
                <dt>Hash mismatch</dt>
                <dd>
                  {formatCount(integrityCheckStore.report.summary.hashMismatch)}
                </dd>
              </div>
              <div>
                <dt>Size mismatch</dt>
                <dd>
                  {formatCount(integrityCheckStore.report.summary.sizeMismatch)}
                </dd>
              </div>
              <div>
                <dt>Type changed</dt>
                <dd>
                  {formatCount(integrityCheckStore.report.summary.typeChanged)}
                </dd>
              </div>
              <div>
                <dt>Unreadable</dt>
                <dd>
                  {formatCount(integrityCheckStore.report.summary.unreadable)}
                </dd>
              </div>
              <div>
                <dt>Changed during check</dt>
                <dd>
                  {formatCount(
                    integrityCheckStore.report.summary.changedDuringCheck,
                  )}
                </dd>
              </div>
              <div>
                <dt>Skipped</dt>
                <dd>
                  {formatCount(integrityCheckStore.report.summary.skipped)}
                </dd>
              </div>
            </dl>
          </section>
        {/if}

        {#if integrityCheckStore.error !== null}
          <section class="panel error-panel">
            <StatusBadge
              label={missingManifest ? "Manifest required" : "Check failed"}
              tone={missingManifest ? "warning" : "danger"}
            />
            <h2>
              {missingManifest
                ? "No integrity record yet"
                : "Something went wrong"}
            </h2>
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

  .action-card {
    display: grid;
    gap: var(--space-md);
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .stack-lg,
  .panel {
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

  h2,
  p {
    margin: 0;
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
</style>
