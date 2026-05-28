import { describe, it, expect } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";
import { createProject, openProject } from "./index";

function makeProject() {
  return {
    id: 123,
    name: "some project",
    formatVersion: 1,
    rootPath: "/some/other/path",
  };
}

describe("createProject", () => {
  it("invokes create_project command", async () => {
    mockIPC((cmd) => {
      if (cmd === "create_project") {
        return makeProject();
      }
    });
    const project = await createProject({
      dbPath: "/some/path.fsdoctor.sqlite",
      name: "some project",
      rootPath: "/some/other/path",
    });
    expect(project.id).toBe(123);
    expect(project.name).toBe("some project");
  });
});

describe("openProject", () => {
  it("invokes open_project command", async () => {
    mockIPC((cmd) => {
      if (cmd === "open_project") {
        return makeProject();
      }
    });
    const project = await openProject({
      dbPath: "/some/path.fsdoctor.sqlite",
    });
    expect(project.id).toBe(123);
    expect(project.name).toBe("some project");
  });
});
