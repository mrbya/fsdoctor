<script lang="ts">
  import type { CommandError, Project } from "$lib/types";
  import { createProject, openProject } from "$lib/api";

  let projectName = $state("My Backup");
  let rootPath = $state("");
  let dbPath = $state("");

  let project = $state<Project | null>(null);
  let errorMessage = $state<string | null>(null);
  let busy = $state(false);

  async function runCreateProject(event: Event) {
    event.preventDefault();
    busy = true;
    errorMessage = null;

    try {
      project = await createProject({
        name: projectName,
        rootPath,
        dbPath,
      });
    } catch (error) {
      errorMessage = formatError(error);
    } finally {
      busy = false;
    }
  }

  async function runOpenProject(event: Event) {
    event.preventDefault();
    busy = true;
    errorMessage = null;

    try {
      project = await openProject({ dbPath });
    } catch (error) {
      errorMessage = formatError(error);
    } finally {
      busy = false;
    }
  }

  function formatError(error: unknown): string {
    const commandError = error as Partial<CommandError>;

    if (typeof commandError.message === "string") {
      return commandError.message;
    }

    return "FSDoctor could not complete the requested operation.";
  }
</script>

<main class="page">
  <section class="card">
    <p class="eyebrow">FSDoctor</p>
    <h1>Project database foundation</h1>
    <p class="muted">
      Phase 1 creates and opens FSDoctor project databases. Scanning is not
      implemented yet.
    </p>
  </section>

  <section class="grid">
    <form class="card" onsubmit={runCreateProject}>
      <h2>Create project</h2>

      <label>
        Project name
        <input bind:value={projectName} placeholder="My Backup" />
      </label>

      <label>
        Backup root path
        <input bind:value={rootPath} placeholder="D:\\Backups\\OldShare" />
      </label>

      <label>
        FSDoctor database path
        <input
          bind:value={dbPath}
          placeholder="C:\\Users\\User\\Documents\\OldShare.fsdoctor.sqlite"
        />
      </label>

      <button type="submit" disabled={busy}>Create project</button>
    </form>

    <form class="card" onsubmit={runOpenProject}>
      <h2>Open project</h2>

      <label>
        FSDoctor database path
        <input bind:value={dbPath} />
      </label>

      <button type="submit" disabled={busy}>Open project</button>
    </form>
  </section>

  {#if errorMessage !== null}
    <section class="card error">
      <h2>Something went wrong</h2>
      <p>{errorMessage}</p>
    </section>
  {/if}

  {#if project !== null}
    <section class="card success">
      <h2>Project loaded</h2>
      <dl>
        <dt>Name</dt>
        <dd>{project.name}</dd>

        <dt>Root path</dt>
        <dd>{project.rootPath}</dd>

        <dt>Format version</dt>
        <dd>{project.formatVersion}</dd>
      </dl>
    </section>
  {/if}
</main>

<style>
  :global(:root) {
    --ctp-rosewater: #f5e0dc;
    --ctp-red: #f38ba8;
    --ctp-peach: #fab387;
    --ctp-yellow: #f9e2af;
    --ctp-green: #a6e3a1;
    --ctp-blue: #89b4fa;
    --ctp-text: #cdd6f4;
    --ctp-subtext0: #a6adc8;
    --ctp-surface0: #313244;
    --ctp-surface1: #45475a;
    --ctp-base: #1e1e2e;
    --ctp-mantle: #181825;
    --ctp-crust: #11111b;

    color: var(--ctp-text);
    background: var(--ctp-base);
    font-family:
      "JetBrainsMono Nerd Font", "JetBrains Mono", ui-monospace, monospace;
  }

  :global(body) {
    margin: 0;
    background: var(--ctp-base);
  }

  .page {
    min-height: 100vh;
    box-sizing: border-box;
    padding: 2rem;
    background:
      radial-gradient(
        circle at top left,
        rgba(137, 180, 250, 0.15),
        transparent 24rem
      ),
      var(--ctp-base);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }

  .card {
    border: 1px solid var(--ctp-surface1);
    border-radius: 1rem;
    padding: 1.25rem;
    background: var(--ctp-mantle);
    box-shadow: 0 1rem 3rem rgba(0, 0, 0, 0.25);
  }

  .eyebrow {
    margin: 0;
    color: var(--ctp-blue);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  h1,
  h2 {
    margin-top: 0.25rem;
  }

  .muted {
    color: var(--ctp-subtext0);
  }

  label {
    display: grid;
    gap: 0.4rem;
    margin-block: 0.9rem;
    color: var(--ctp-subtext0);
  }

  input {
    border: 1px solid var(--ctp-surface1);
    border-radius: 0.6rem;
    padding: 0.7rem;
    color: var(--ctp-text);
    background: var(--ctp-crust);
    font: inherit;
  }

  button {
    border: 0;
    border-radius: 0.6rem;
    padding: 0.75rem 1rem;
    color: var(--ctp-crust);
    background: var(--ctp-blue);
    font: inherit;
    cursor: pointer;
  }

  button:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  .error {
    margin-top: 1rem;
    border-color: var(--ctp-red);
  }

  .success {
    margin-top: 1rem;
    border-color: var(--ctp-green);
  }

  dl {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 0.5rem 1rem;
  }

  dt {
    color: var(--ctp-subtext0);
  }

  dd {
    margin: 0;
  }
</style>
