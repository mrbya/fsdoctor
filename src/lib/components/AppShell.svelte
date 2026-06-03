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

  let activeView = $state<View>("create-manifest");

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
            class="nav-btn"
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
    grid-template-columns: 18rem minmax(0, 1fr);
    min-height: 100vh;
  }

  .sidebar {
    position: sticky;
    top: 0;
    display: grid;
    align-content: start;
    gap: var(--fd-space-lg);
    height: 100vh;
    border-right: 1px solid var(--fd-color-border);
    padding: var(--fd-space-lg);
    background: color-mix(in srgb, var(--fd-color-bg-muted), transparent 12%);
  }

  .content {
    min-width: 0;
    padding: var(--fd-space-xl);
  }

  @media (max-width: 48rem) {
    .shell {
      grid-template-columns: 1fr;
    }

    .sidebar {
      position: static;
      height: auto;
    }

    nav {
      grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr));
    }
  }
</style>
