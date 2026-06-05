<script lang="ts">
  import {
    ClipboardList,
    ClipboardPlus,
    FolderCheck,
    LayoutDashboard,
    SettingsIcon,
  } from "@lucide/svelte";
  import CreateManifest from "$lib/views/CreateManifest.svelte";
  import CheckBackup from "$lib/views/CheckBackup.svelte";
  import ReportHistory from "$lib/views/ReportHistory.svelte";
  import Settings from "$lib/views/Settings.svelte";
  import Dashboard from "$lib/views/Dashboard.svelte";

  type View =
    | "dashboard"
    | "create-manifest"
    | "check-backup"
    | "reports"
    | "settings";

  let activeView = $state<View>("dashboard");

  const navItems = $derived([
    { id: "dashboard" as View, icon: LayoutDashboard, label: "Dashboard" },
    {
      id: "create-manifest" as View,
      icon: ClipboardPlus,
      label: "Create manifest",
    },
    { id: "check-backup" as View, icon: FolderCheck, label: "Check backup" },
    { id: "reports" as View, icon: ClipboardList, label: "Reports" },
    { id: "settings" as View, icon: SettingsIcon, label: "Settings" },
  ]);
</script>

<div class="shell">
  <nav class="sidebar">
    <div class="sidebar-inner">
      <ul class="nav-list">
        {#each navItems as item (item.id)}
          <li>
            <button
              class="nav-button"
              class:active={activeView === item.id}
              aria-current={activeView === item.id ? "page" : undefined}
              aria-label={item.label}
              title={item.label}
              onclick={() => (activeView = item.id)}
            >
              <span class="icon">
                <item.icon size={18} strokeWidth={1.75} />
              </span>
            </button>
          </li>
        {/each}
      </ul>
    </div>
  </nav>

  <main class="content">
    <div class="content-inner">
      {#if activeView === "dashboard"}
        <Dashboard />
      {:else if activeView === "create-manifest"}
        <CreateManifest />
      {:else if activeView === "check-backup"}
        <CheckBackup />
      {:else if activeView === "reports"}
        <ReportHistory />
      {:else if activeView === "settings"}
        <Settings />
      {/if}
    </div>
  </main>
</div>

<style>
  .shell {
    display: grid;
    grid-template-columns: 4rem minmax(0, 1fr);
    min-height: 100vh;
  }

  .sidebar {
    flex-shrink: 0;
    background: var(--bg-raised);
    border-right: 1px solid var(--border);
  }

  .sidebar-inner {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-md);
    padding: 0.75rem 0.5rem;
  }

  .nav-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 100%;
  }

  .nav-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    min-height: 2.5rem;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius);
    color: var(--text-muted);
    cursor: pointer;
    transition:
      background 0.15s,
      color 0.15s,
      border-color 0.15s;
  }

  .nav-button:hover {
    background: var(--bg-input);
    border-color: var(--border);
    color: var(--text);
  }

  .nav-button.active {
    background: color-mix(in srgb, var(--accent) 12%, var(--bg-input));
    border-color: color-mix(in srgb, var(--accent) 32%, var(--border));
    color: var(--accent);
  }

  .icon {
    line-height: 1;
    display: flex;
  }

  .content {
    overflow-y: auto;
    background: var(--bg);
    padding: var(--space-lg);
  }

  .content-inner {
    width: min(72rem, 100%);
    margin: 0 auto;
  }

  @media (max-width: 48rem) {
    .shell {
      grid-template-columns: 1fr;
    }

    .sidebar {
      width: 100%;
      border-right: 0;
      border-bottom: 1px solid var(--border);
    }

    .sidebar-inner {
      flex-direction: row;
      justify-content: space-between;
      padding: var(--space-sm);
    }

    .nav-list {
      flex-direction: row;
      justify-content: flex-end;
      gap: var(--space-2xs);
    }

    .nav-button {
      min-width: 2.75rem;
      min-height: 2.75rem;
    }

    .content {
      padding: var(--space-md);
    }
  }
</style>
