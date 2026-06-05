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

/**
 * Manifest generation phase.
 */
export type ManifestGenerationPhase =
  | "walking_and_hashing"
  | "writing"
  | "finishing";

/**
 * Manifest generation progress payload.
 */
export type ManifestGenerationProgress = {
  /** Current phase. */
  phase: ManifestGenerationPhase;
  /** Current path, if available. */
  currentPath: string | null;
  /** Files seen. */
  filesSeen: number;
  /** Directories seen. */
  dirsSeen: number;
  /** Bytes seen. */
  bytesSeen: number;
  /** Files hashed. */
  filesHashed: number;
  /** Bytes hashed. */
  bytesHashed: number;
  /** Unreadable entries. */
  unreadableEntries: number;
  /** Changed-during-scan entries. */
  changedDuringScan: number;
  /** Entries written to the database. */
  resultsWritten: number;
};

/**
 * Event emitted while manifest generation is running.
 */
export type ManifestGenerationProgressEvent = {
  /** Job identifier. */
  jobId: string;
  /** Progress snapshot. */
  progress: ManifestGenerationProgress;
};

/**
 * Request to start an integrity check.
 */
export type StartIntegrityCheckRequest = {
  /** FSDoctor project database path. */
  dbPath: string;
  /** Optional DB batch size. */
  dbBatchSize?: number;
};

/**
 * Integrity check final status.
 */
export type IntegrityCheckEventStatus = "completed" | "cancelled" | "failed";

/**
 * Integrity check summary.
 */
export type IntegrityCheckSummary = {
  /**  OK entries. */
  ok: number;
  /**  Missing entries. */
  missing: number;
  /**  New entries. */
  new: number;
  /**  Hash mismatches. */
  hashMismatch: number;
  /**  Size mismatches. */
  sizeMismatch: number;
  /**  Type changes. */
  typeChanged: number;
  /**  Unreadable entries. */
  unreadable: number;
  /**  Changed-during-check entries. */
  changedDuringCheck: number;
  /**  Skipped entries. */
  skipped: number;
};

/**
 * Integrity check report payload.
 */
export type IntegrityCheckReport = {
  /**  Integrity-check scan id. */
  scanId: number;
  /**  Manifest-generation scan id used as baseline. */
  manifestScanId: number;
  /**  Result summary. */
  summary: IntegrityCheckSummary;
};

/**
 * Integrity check phase.
 */
export type IntegrityCheckPhase =
  | "loading_manifest"
  | "walking_and_checking"
  | "recording_missing_entries"
  | "writing"
  | "finishing";

/**
 * Integrity check progress payload.
 */
export type IntegrityCheckProgress = {
  /**  Current phase. */
  phase: IntegrityCheckPhase;
  /**  Current path, if available. */
  currentPath: string | null;
  /**  Current summary. */
  summary: IntegrityCheckSummary;
  /**  Files seen. */
  filesSeen: number;
  /**  Directories seen. */
  dirsSeen: number;
  /**  Bytes seen. */
  bytesSeen: number;
  /**  Files hashed. */
  filesHashed: number;
  /**  Results written. */
  resultsWritten: number;
};

/**
 * Event emitted while integrity check is running.
 */
export type IntegrityCheckProgressEvent = {
  /**  Job identifier. */
  jobId: string;
  /**  Progress snapshot. */
  progress: IntegrityCheckProgress;
};

/**
 * Event emitted when integrity check finishes.
 */
export type IntegrityCheckFinishedEvent = {
  /**  Job identifier. */
  jobId: string;
  /**  Final job status. */
  status: IntegrityCheckEventStatus;
  /**  Report if the job reached core integrity-check completion. */
  report: IntegrityCheckReport | null;
  /**  Error if the job failed. */
  error: CommandError | null;
};
