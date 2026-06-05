/**
 * Project metada contained in a project database.
 */
export type Project = {
  /** Database-local project id. */
  id: number;
  /** User-facing project name. */
  name: string;
  /** Backup root path. */
  rootPath: string;
  /** Database format version. */
  formatVersion: number;
};

/**
 * Request to create a new FSDoctor project database.
 */
export type CreateProjectRequest = {
  /** Database path. */
  dbPath: string;
  /** User-facing project name. */
  name: string;
  /** Backup root path. */
  rootPath: string;
};

/**
 * Request to open an existing FSDoctor project database.
 */
export type OpenProjectRequest = {
  /** Database path. */
  dbPath: string;
};

/**
 * Type containing error data thrown by Tauri commands.
 */
export type CommandError = {
  /** Stable machine-readable error kind. */
  kind: string;
  /** User-facing error message. */
  message: string;
  /** Optional technical details. */
  details?: string | null;
};

/**
 * Request to start manifest generation.
 */
export type StartManifestGenerationRequest = {
  /** FSDoctor project database path. */
  dbPath: string;
  /** Optional DB batch size. */
  dbBatchSize?: number;
};

/**
 * Returned after a background job starts.
 */
export type JobStarted = {
  /** Job identifier. */
  jobId: string;
};

/**
 * Request to cancel a background job.
 */
export type CancelJobRequest = {
  /** Job identifier. */
  jobId: string;
};

/**
 * Result of requesting job cancellation.
 */
export type CancelJobResult = {
  /** Job identifier. */
  jobId: string;
  /** Whether a matching job was found and cancellation was requested. */
  cancellationRequested: boolean;
};

/**
 * Manifest generation final status.
 */
export type ManifestGenerationEventStatus =
  | "completed"
  | "cancelled"
  | "failed";

/**
 * Manifest generation report payload.
 */
export type ManifestGenerationReport = {
  /** Scan id. */
  scanId: number;
  /** Total directories seen. */
  totalDirs: number;
  /** Total files seen. */
  totalFiles: number;
  /** Total symlinks seen. */
  totalSymlinks: number;
  /** Total other entries seen. */
  totalOther: number;
  /** Total bytes seen. */
  totalBytes: number;
  /** Files successfully hashed. */
  hashedFiles: number;
  /** Unreadable entries. */
  unreadableEntries: number;
  /** Files changed during scan. */
  changedDuringScan: number;
};

/**
 * Event emitted when manifest generation finishes.
 */
export type ManifestGenerationFinishedEvent = {
  /** Job identifier. */
  jobId: string;
  /** Final job status. */
  status: ManifestGenerationEventStatus | null;
  /** Report if the job reached core manifest generation completion. */
  report: ManifestGenerationReport | null;
  /** Error if the job failed. */
  error: CommandError | null;
};
