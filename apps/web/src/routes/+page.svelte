<script lang="ts">
  import { resolve } from '$app/paths';
  import { NAVIGATION } from '$lib/ui/copy/navigation';
</script>

<svelte:head>
  <title>{NAVIGATION.brand} — {NAVIGATION.siteDescription}</title>
</svelte:head>

<!-- Transitional site entry; Site D is a separate F0.5 delivery. -->
<div class="entry">
  <h1>{NAVIGATION.brand}</h1>
  <p>{NAVIGATION.siteDescription}</p>
  <section aria-labelledby="available-title">
    <h2 id="available-title">{NAVIGATION.available}</h2>
    <p>{NAVIGATION.siteNotice}</p>
    <a href={resolve('/lab')}>{NAVIGATION.openLab}</a>
  </section>
</div>

<style>
  .entry {
    max-width: 800px;
    padding: var(--space-8) var(--space-4);
    margin: auto;
  }

  h1 {
    font-size: var(--text-display);
    letter-spacing: -0.03em;
    margin: 0;
  }

  p {
    color: var(--color-text-muted);
  }

  section {
    margin-top: var(--space-8);
    padding: var(--space-6);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xl);
    background: var(--color-surface);
  }

  h2 {
    font-size: var(--text-lg);
    margin: 0;
  }

  a {
    display: inline-block;
    padding: var(--space-2) var(--space-3);
    color: var(--color-on-accent);
    background: var(--color-accent);
    border-radius: var(--radius-md);
    text-decoration: none;
  }

  a:hover {
    background: var(--color-accent-hover);
  }

  a:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: 3px;
  }
</style>
