<!-- Modules and their topics; the current page is highlighted. -->
<script lang="ts">
  import { page } from '$app/state';
  import Icon from '$lib/ui/components/Icon.svelte';
  import { MODULES, UPCOMING_MODULES } from '$lib/ui/copy/navigation';

  const currentPath = $derived(page.url.pathname);
</script>

<nav aria-label="Módulos" class="nav">
  <a href="/" class="home" aria-current={currentPath === '/' ? 'page' : undefined}>
    <Icon name="home" />
    Início
  </a>

  {#each MODULES as module (module.title)}
    <section class="module">
      <p class="module-title">
        <Icon name={module.icon} size={14} />
        {module.title}
      </p>
      <ul class="topics">
        {#each module.topics as topic (topic.href)}
          <li>
            <a
              href={topic.href}
              class="topic"
              aria-current={currentPath.startsWith(topic.href) ? 'page' : undefined}
            >
              {topic.title}
            </a>
          </li>
        {/each}
      </ul>
    </section>
  {/each}

  <section class="module upcoming">
    <p class="module-title">Em breve</p>
    <ul class="topics">
      {#each UPCOMING_MODULES as module (module.title)}
        <li class="topic disabled">
          <Icon name={module.icon} size={14} />
          {module.title}
        </li>
      {/each}
    </ul>
  </section>
</nav>

<style>
  .nav {
    display: grid;
    gap: var(--space-4);
  }

  .home,
  .topic {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    min-height: 32px;
    padding: 0 var(--space-2);
    color: var(--color-text-muted);
    text-decoration: none;
    border-radius: var(--radius-md);
    transition:
      background var(--duration-fast) var(--easing-standard),
      color var(--duration-fast) var(--easing-standard);
  }

  .home:hover,
  a.topic:hover {
    color: var(--color-text);
    background: var(--color-surface-hover);
  }

  .home[aria-current='page'],
  .topic[aria-current='page'] {
    color: var(--color-accent);
    font-weight: 500;
    background: var(--color-accent-soft);
  }

  .home:focus-visible,
  .topic:focus-visible {
    outline: 2px solid var(--color-accent);
  }

  .module-title {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin: 0 0 var(--space-1);
    padding: 0 var(--space-2);
    font-size: var(--text-xs);
    font-weight: 600;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .topics {
    display: grid;
    gap: 2px;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .topic {
    padding-left: calc(var(--space-2) + 22px);
  }

  .upcoming .topic {
    padding-left: var(--space-2);
  }

  .disabled {
    opacity: 0.55;
  }
</style>
