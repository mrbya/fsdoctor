import type { CommandError } from "$lib/types";

const numberFormatter = new Intl.NumberFormat();

export function normalizeCommandError(error: unknown): CommandError {
  const candidate = error as Partial<CommandError>;

  if (
    typeof candidate.message === "string" &&
    typeof candidate.kind === "string"
  ) {
    return {
      kind: candidate.kind,
      message: candidate.message,
      details: candidate.details ?? null,
    };
  }

  return {
    kind: "unknown",
    message: "FSDoctor could not complete the requested operation.",
    details: null,
  };
}

/**
 * Returns whether a candidate path appears to be inside a root path.
 *
 * This is a frontend safety warning only. The backend remains the authority for
 * native path handling.
 */
export function isPathInsideRoot(
  candidatePath: string,
  rootPath: string,
): boolean {
  const candidate = normalizePathForWarning(candidatePath);
  const root = normalizePathForWarning(rootPath);

  if (candidate.length === 0 || root.length === 0) {
    return false;
  }

  return candidate === root || candidate.startsWith(`${root}/`);
}

export function formatCount(value: number): string {
  return numberFormatter.format(value);
}

export function formatBytes(bytes: number): string {
  if (bytes < 1024) {
    return `${formatCount(bytes)} B`;
  }

  const units = ["KB", "MB", "GB", "TB", "PB"];
  let value = bytes;
  let unitIndex = -1;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }

  return `${value.toFixed(value >= 10 ? 0 : 1)} ${units[unitIndex]}`;
}

function normalizePathForWarning(path: string): string {
  return path
    .trim()
    .replaceAll("\\", "/")
    .replace(/\/+$/u, "")
    .toLocaleLowerCase();
}
