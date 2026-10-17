<!--
  Application frame: sidebar (brand, modules, theme) + the current page.
  See docs/04-design/layout-e-telas.md.
-->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import { afterNavigate } from '$app/navigation';
  import { NAVIGATION } from '$lib/ui/copy/navigation';

  interface Props {
    sidebar: Snippet;
    children: Snippet;
  }

  let { sidebar, children }: Props = $props();
  let menuOpen = $state(false);

  afterNavigate(() => {
    menuOpen = false;
  });
</script>

<a class="skip" href="#main-content">{NAVIGATION.skip}</a>
<div class="shell">
  <header class="mobile-header">
    <span>{NAVIGATION.brand}</span>
    <button
      type="button"
      aria-expanded={menuOpen}
      aria-controls="module-sidebar"
      onclick={() => (menuOpen = !menuOpen)}
    >
      {NAVIGATION.menu}
    </button>
  </header>
  <aside id="module-sidebar" class="sidebar" class:open={menuOpen}>{@render sidebar()}</aside>
  <main id="main-content" tabindex="-1" class="page">{@render children()}</main>
</div>

<style>
  .skip {
    position: fixed;
    top: var(--space-2);
    left: var(--space-2);
    z-index: 10;
    padding: var(--space-2) var(--space-3);
    color: var(--color-on-accent);
    background: var(--color-accent);
    border-radius: var(--radius-md);
    transform: translateY(-200%);
  }

  .skip:focus {
    transform: translateY(0);
  }

  .mobile-header {
    display: none;
  }

  .shell {
    display: grid;
    grid-template-columns: var(--layout-sidebar-width) minmax(0, 1fr);
    height: 100dvh;
    background: var(--color-bg);
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    padding: var(--space-4) var(--space-3);
    background: var(--color-surface);
    border-right: 1px solid var(--color-border);
    overflow-y: auto;
  }

  .page {
    min-width: 0;
    min-height: 0;
    overflow: auto;
  }

  @media (max-width: 899px) {
    .shell {
      display: flex;
      flex-direction: column;
      height: auto;
      min-height: 100dvh;
    }

    .page {
      flex: 1;
      overflow: visible;
    }

    .sidebar {
      display: none;
      border-right: 0;
      border-bottom: 1px solid var(--color-border);
    }

    .sidebar.open {
      display: flex;
    }

    .mobile-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: var(--space-2);
      padding: var(--space-3) var(--space-4);
      background: var(--color-surface);
      border-bottom: 1px solid var(--color-border);
    }

    button {
      padding: var(--space-2);
      font: inherit;
      color: var(--color-text);
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      cursor: pointer;
    }

    button:hover {
      background: var(--color-surface-hover);
    }

    button:focus-visible {
      outline: 2px solid var(--color-accent);
      outline-offset: 2px;
    }
  }
</style>
