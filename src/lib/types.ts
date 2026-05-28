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
