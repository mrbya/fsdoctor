import { createProject, openProject } from "$lib/api";
import type {
  CommandError,
  CreateProjectRequest,
  OpenProjectRequest,
  Project,
} from "$lib/types";
import { normalizeCommandError } from "$lib/utils/helpers";

class ProjectStore {
  project = $state<Project | null>(null);
  busy = $state(false);
  error = $state<CommandError | null>(null);

  async create(request: CreateProjectRequest): Promise<void> {
    this.busy = true;
    this.error = null;

    try {
      this.project = await createProject(request);
    } catch (error) {
      this.error = normalizeCommandError(error);
    } finally {
      this.busy = false;
    }
  }

  async open(request: OpenProjectRequest): Promise<void> {
    this.busy = true;
    this.error = null;

    try {
      this.project = await openProject(request);
    } catch (error) {
      this.error = normalizeCommandError(error);
    } finally {
      this.busy = false;
    }
  }

  clearError(): void {
    this.error = null;
  }
}

export const projectStore = new ProjectStore();
