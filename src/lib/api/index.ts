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
