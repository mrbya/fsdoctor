import { cancelJob, startManifestGeneration } from "$lib/api";
import type {
  CommandError,
  ManifestGenerationFinishedEvent,
  ManifestGenerationProgress,
  ManifestGenerationProgressEvent,
  ManifestGenerationReport,
} from "$lib/types";
import { normalizeCommandError } from "$lib/utils/helpers";
import { listen } from "@tauri-apps/api/event";

type ManifestGenerationStatus =
  | "idle"
  | "running"
  | "cancelling"
  | "completed"
  | "cancelled"
  | "failed";

class ManifestGenerationStore {
  jobId = $state<string | null>(null);
  status = $state<ManifestGenerationStatus>("idle");
  progress = $state<ManifestGenerationProgress | null>(null);
  report = $state<ManifestGenerationReport | null>(null);
  error = $state<CommandError | null>(null);

  private unlistenProgress: (() => void) | null = null;
  private unlistenFinished: (() => void) | null = null;

  get isActive(): boolean {
    return this.status === "running" || this.status === "cancelling";
  }

  async init(): Promise<void> {
    if (this.unlistenProgress === null) {
      this.unlistenProgress = await listen<ManifestGenerationProgressEvent>(
        "manifest-generation-progress",
        (event) => {
          this.handleProgressEvent(event.payload);
        },
      );
    }

    if (this.unlistenFinished === null) {
      this.unlistenFinished = await listen<ManifestGenerationFinishedEvent>(
        "manifest-generation-finished",
        (event) => {
          this.handleFinishedEvent(event.payload);
        },
      );
    }
  }

  async start(dbPath: string): Promise<void> {
    await this.init();

    this.status = "running";
    this.progress = null;
    this.report = null;
    this.error = null;

    try {
      const started = await startManifestGeneration({ dbPath });
      this.jobId = started.jobId;
    } catch (error) {
      this.status = "failed";
      this.error = normalizeCommandError(error);
      this.jobId = null;
    }
  }

  async cancel(): Promise<void> {
    if (this.jobId === null) {
      return;
    }

    this.status = "cancelling";

    try {
      await cancelJob({ jobId: this.jobId });
    } catch (error) {
      this.status = "failed";
      this.error = normalizeCommandError(error);
    }
  }

  clear(): void {
    this.jobId = null;
    this.status = "idle";
    this.progress = null;
    this.report = null;
    this.error = null;
  }

  dispose(): void {
    if (this.unlistenProgress !== null) {
      this.unlistenProgress();
      this.unlistenProgress = null;
    }

    if (this.unlistenFinished !== null) {
      this.unlistenFinished();
      this.unlistenFinished = null;
    }
  }

  private handleProgressEvent(event: ManifestGenerationProgressEvent): void {
    if (event.jobId !== this.jobId) {
      return;
    }

    this.progress = event.progress;
  }

  private handleFinishedEvent(event: ManifestGenerationFinishedEvent): void {
    if (event.jobId !== this.jobId) {
      return;
    }

    this.report = event.report;
    this.error = event.error;

    if (event.status === "completed") {
      this.status = "completed";
    } else if (event.status === "cancelled") {
      this.status = "cancelled";
    } else {
      this.status = "failed";
    }
  }
}

export const manifestGenerationStore = new ManifestGenerationStore();
