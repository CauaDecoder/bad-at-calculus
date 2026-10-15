<!--
  Composition root of the UI: creates the engines and the theme once and
  shares them with every page through Svelte context.
-->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import '@fontsource-variable/inter';
  import '@fontsource-variable/jetbrains-mono';
  import 'katex/dist/katex.min.css';
  import '../styles/tokens.css';
  import '../styles/base.css';
  import { LocalPreferenceStore } from '$lib/adapters/local-preference-store';
  import { ThemeController } from '$lib/application/theme.svelte';
  import { createEngines, provideEngines } from '$lib/composition/engines';
  import IconButton from '$lib/ui/components/IconButton.svelte';
  import AppShell from '$lib/ui/layout/AppShell.svelte';
  import ModuleNav from '$lib/ui/layout/ModuleNav.svelte';
  import { provideTheme } from '$lib/ui/theme-context';
  import { page } from '$app/state';

  let { children }: { children: Snippet } = $props();

  // PROTOTYPE: design prototypes bring their own shell (docs/references/design/README.md).
  const isPrototype = $derived(page.url.pathname.startsWith('/prototype'));

  const theme = new ThemeController(
    new LocalPreferenceStore(),
    window.matchMedia('(prefers-color-scheme: dark)').matches,
  );
  provideTheme(theme);
  provideEngines(createEngines());

  $effect(() => {
    document.documentElement.dataset.theme = theme.theme;
  });
</script>

{#if isPrototype}
  {@render children()}
{:else}
  <AppShell>
    {#snippet sidebar()}
      <a href="/" class="brand">
        <span class="logo" aria-hidden="true">f′</span>
        <span class="brand-name">Math View</span>
      </a>

      <ModuleNav />

      <footer class="sidebar-footer">
        <span class="version">v0.3</span>
        <IconButton
          icon={theme.theme === 'dark' ? 'sun' : 'moon'}
          label={theme.theme === 'dark' ? 'Usar tema claro' : 'Usar tema escuro'}
          onclick={() => theme.toggle()}
        />
      </footer>
    {/snippet}

    {@render children()}
  </AppShell>
{/if}

<style>
  .brand {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: 0 var(--space-2);
    margin-bottom: var(--space-6);
    color: var(--color-text);
    text-decoration: none;
  }

  .logo {
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    font-family: var(--font-mono);
    font-weight: 600;
    font-size: var(--text-sm);
    color: #fff;
    background: var(--color-accent);
    border-radius: var(--radius-md);
  }

  .brand-name {
    font-weight: 600;
    font-size: var(--text-md);
  }

  .sidebar-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: auto;
    padding: var(--space-4) var(--space-2) 0;
  }

  .version {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
