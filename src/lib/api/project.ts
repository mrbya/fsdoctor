import { invoke } from "@tauri-apps/api/core";

export type Project = {
  id: number;
  name: string;
  rootPath: string;
  formatVersion: number;
};

export type CreateProjectRequest = {
  dbPath: string;
  name: string;
  rootPath: string;
};

export type OpenProjectRequest = {
  dbPath: string;
};

export type CommandError = {
  kind: string;
  message: string;
  details?: string | null;
};

export async function createProject(
  request: CreateProjectRequest,
): Promise<Project> {
  return invoke<Project>("create_project", { request });
}

export async function openProject(
  request: OpenProjectRequest,
): Promise<Project> {
  return invoke<Project>("open_project", { request });
}
