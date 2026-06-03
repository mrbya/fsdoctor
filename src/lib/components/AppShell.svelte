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
    <ul class="nav-list">
      {#each navItems as item (item.id)}
        <li>
          <button
            class="nav-button"
            class:active={activeView === item.id}
            title={item.label}
            onclick={() => (activeView = item.id)}
          >
            <span class="icon">
              <item.icon size={20} strokeWidth={1.75} />
            </span>
          </button>
        </li>
      {/each}
    </ul>
  </nav>

  <main class="content">
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
  </main>
</div>

<style>
  .shell {
    display: grid;
    grid-template-columns: 4.5rem minmax(0, 1fr);
    min-height: 100vh;
  }

  .sidebar {
    width: 64px;
    flex-shrink: 0;
    background: var(--bg-raised);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 0.5rem 0;
  }

  .nav-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0 0.5rem;
  }

  .nav-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    aspect-ratio: 1;
    background: transparent;
    border: none;
    border-radius: var(--radius);
    color: var(--text-muted);
    cursor: pointer;
    transition:
      background 0.15s,
      color 0.15s;
  }

  .nav-button:hover {
    background: var(--bg-input);
    color: var(--text);
  }

  .nav-button.active {
    background: color-mix(in srgb, var(--accent) 15%, transparent);
    color: var(--accent);
  }

  .icon {
    line-height: 1;
    display: flex;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    background: var(--bg);
    padding: var(--space--sm);
  }
</style>
