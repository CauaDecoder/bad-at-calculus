<!--
  Application frame: sidebar (brand, modules, theme) + the current page.
  See docs/04-design/layout-e-telas.md.
-->
<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    sidebar: Snippet;
    children: Snippet;
  }

  let { sidebar, children }: Props = $props();
</script>

<div class="shell">
  <aside class="sidebar">{@render sidebar()}</aside>
  <main class="page">{@render children()}</main>
</div>

<style>
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

  @media (max-width: 960px) {
    .shell {
      grid-template-columns: minmax(0, 1fr);
      grid-template-rows: auto 1fr;
      height: auto;
      min-height: 100dvh;
    }

    .sidebar {
      border-right: 0;
      border-bottom: 1px solid var(--color-border);
    }
  }
</style>
