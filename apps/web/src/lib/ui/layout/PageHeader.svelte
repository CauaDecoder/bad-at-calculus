<!-- Title of a topic page with a one-line explanation. -->
<script lang="ts">
  interface Props {
    module: string;
    title: string;
    description: string;
  }

  let { module, title, description }: Props = $props();
</script>

<header class="header">
  <p class="module">{module}</p>
  <h1 class="title">{title}</h1>
  <p class="description">{description}</p>
</header>

<style>
  .header {
    padding: var(--space-4) var(--space-4) 0;
  }

  .module {
    margin: 0;
    font-size: var(--text-xs);
    font-weight: 600;
    color: var(--color-accent);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .title {
    margin: var(--space-1) 0;
    font-size: var(--text-xl);
    font-weight: 600;
    letter-spacing: -0.01em;
  }

  .description {
    margin: 0;
    max-width: 72ch;
    color: var(--color-text-muted);
  }
</style>
