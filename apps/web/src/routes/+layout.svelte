<!--
  Composition root of the UI: creates the engines and the theme once and
  shares them with every page through Svelte context.
-->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import { resolve } from '$app/paths';
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
  import { NAVIGATION } from '$lib/ui/copy/navigation';
  import { provideTheme } from '$lib/ui/theme-context';

  let { children }: { children: Snippet } = $props();

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

<AppShell>
  {#snippet sidebar()}
    <a href={resolve('/')} class="brand">
      <span class="logo" aria-hidden="true">V</span>
      <span class="brand-name">{NAVIGATION.brand}</span>
    </a>

    <ModuleNav />

    <footer class="sidebar-footer">
      <span class="version">{NAVIGATION.lab}</span>
      <IconButton
        icon={theme.theme === 'dark' ? 'sun' : 'moon'}
        label={theme.theme === 'dark' ? NAVIGATION.lightTheme : NAVIGATION.darkTheme}
        onclick={() => theme.toggle()}
      />
    </footer>
  {/snippet}

  {@render children()}
</AppShell>

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
    color: var(--color-on-accent);
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
