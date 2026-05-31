<script lang="ts">
  import { page } from "$app/state";
  import type { Snippet } from "svelte";

  type NavItem = {
    href: string;
    label: string;
    description: string;
  };

  const navItems: NavItem[] = [
    {
      href: "/",
      label: "Dashboard",
      description: "Project overview",
    },
    {
      href: "/create-manifest",
      label: "Create Manifest",
      description: "Create an integrity record",
    },
    {
      href: "/check-backup",
      label: "Check Backup",
      description: "Verify backup health",
    },
    {
      href: "/reports",
      label: "Reports",
      description: "Review previous checks",
    },
    {
      href: "/settings",
      label: "Settings",
      description: "Application preferences",
    },
  ];

  let { children }: { children: Snippet } = $props();

  function isActive(href: string): boolean {
    if (href === "/") {
      return page.url.pathname === "/";
    }

    return page.url.pathname.startsWith(href);
  }
</script>

<div class="shell">
  <aside class="sidebar">
    <a href="/" class="brand">
      <span class="brand-mark">FS</span>
      <span>
        <strong>FSDoctor</strong>
        <small>Backup health</small>
      </span>
    </a>

    <nav aria-label="Main navigation">
      {#each navItems as item}
        <a href={item.href} class:active={isActive(item.href)}>
          <span>{item.label}</span>
          <small>{item.description}</small>
        </a>
      {/each}
    </nav>
  </aside>

  <main class="content">
    {@render children()}
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

  .brand {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: var(--fd-space-sm);
    color: var(--fd-color-text);
    text-decoration: none;
  }

  .brand-mark {
    display: grid;
    place-items: center;
    width: 2.5rem;
    aspect-ratio: 1;
    border-radius: var(--fd-radius-md);
    color: var(--ctp-crust);
    background: var(--fd-color-accent);
    font-weight: 700;
  }

  .brand strong,
  .brand small {
    display: block;
  }

  .brand small {
    color: var(--fd-color-text-muted);
  }

  nav {
    display: grid;
    gap: var(--fd-space-xs);
  }

  nav a {
    display: grid;
    gap: 0.15rem;
    border: 1px solid transparent;
    border-radius: var(--fd-radius-md);
    padding: var(--fd-space-sm);
    color: var(--fd-color-text-muted);
    text-decoration: none;
  }

  nav a:hover,
  nav a.active {
    border-color: var(--fd-color-border);
    color: var(--fd-color-text);
    background: var(--fd-color-bg-raised);
  }

  nav small {
    color: var(--fd-color-text-subtle);
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
