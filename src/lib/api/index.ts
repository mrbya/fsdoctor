/**
 * Single invoke boundary - all Tauri command calls live here.
 *
 * Views and stores must import from this file, never call `invoke()` directly.
 * Grouping all IPC calls in one place makes the contract between the frontend
 * and the Rust backend visible at a glance and simplifies mocking in tests.
 */
import { invoke } from "@tauri-apps/api/core";

import type {
  Project,
  CreateProjectRequest,
  OpenProjectRequest,
  StartManifestGenerationRequest,
  JobStarted,
  CancelJobRequest,
  CancelJobResult,
  StartIntegrityCheckRequest,
} from "$lib/types";

/**
 * Creates a new FSDoctor project database.
 *
 * @returns Project metadata on successful db creation and commit.
 * @throws If the underlying backend db client operations fail.
 */
export async function createProject(
  request: CreateProjectRequest,
): Promise<Project> {
  return invoke<Project>("create_project", { request });
}

/**
 * Opens an existing FSDoctor project database.
 *
 * @returns Project metadata on successful db opening and query.
 * @throws If the underlying backed db client operations fail.
 */
export async function openProject(
  request: OpenProjectRequest,
): Promise<Project> {
  return invoke<Project>("open_project", { request });
}

/**
 * Starts manifest generation as a backend job.
 *
 * @returns Job id of the started manifest generation job.
 * @throws If the backend fails to start the job.
 */
export async function startManifestGeneration(
  request: StartManifestGenerationRequest,
): Promise<JobStarted> {
  return invoke<JobStarted>("start_manifest_generation", { request });
}

/**
 * Requests cancellation of a background job.
 *
 * @returns Cancellation result.
 * @throws If the backend cannot access job state.
 */
export async function cancelJob(
  request: CancelJobRequest,
): Promise<CancelJobResult> {
  return invoke<CancelJobResult>("cancel_job", { request });
}

/**
 * Starts integrity check as a backend job.
 *
 * @returns Job id of the started integrity check job.
 * @throws If the backend fails to start the job.
 */
export async function startIntegrityCheck(
  request: StartIntegrityCheckRequest,
): Promise<JobStarted> {
  return invoke<JobStarted>("start_integrity_check", { request });
}
