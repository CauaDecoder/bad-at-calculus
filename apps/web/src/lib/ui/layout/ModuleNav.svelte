<!-- Modules and their topics; the current page is highlighted. -->
<script lang="ts">
  import { page } from '$app/state';
  import { resolve } from '$app/paths';
  import Icon from '$lib/ui/components/Icon.svelte';
  import { MODULES, NAVIGATION, UPCOMING_MODULES } from '$lib/ui/copy/navigation';

  const currentPath = $derived(page.url.pathname);
  const inLab = $derived(
    currentPath === resolve('/lab') || currentPath.startsWith(resolve('/lab') + '/'),
  );
</script>

<nav aria-label={NAVIGATION.study} class="nav">
  <p class="module-title">{NAVIGATION.study}</p>
  <a
    href={resolve('/lab')}
    class="home"
    aria-current={currentPath === resolve('/lab') ? 'page' : undefined}
  >
    <Icon name="calculus" />
    {NAVIGATION.lab}
  </a>

  {#if inLab}
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
                href={resolve(topic.href)}
                class="topic"
                aria-current={currentPath === resolve(topic.href) ? 'page' : undefined}
              >
                {topic.title}
              </a>
            </li>
          {/each}
        </ul>
      </section>
    {/each}
  {/if}

  <section class="module upcoming">
    <p class="module-title">{NAVIGATION.upcoming}</p>
    <ul class="topics">
      {#each UPCOMING_MODULES as module (module.title)}
        <li class="planned">
          <span>{module.title}</span>
          <span class="phase">{NAVIGATION.planned} · {module.phase}</span>
        </li>
      {/each}
    </ul>
  </section>

  <a
    href={resolve('/')}
    class="home"
    aria-current={currentPath === resolve('/') ? 'page' : undefined}
  >
    <Icon name="home" />
    {NAVIGATION.site}
  </a>
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

  .planned {
    display: grid;
    gap: var(--space-1);
    padding: var(--space-2);
    color: var(--color-text-muted);
  }

  .phase {
    font-size: var(--text-xs);
  }
</style>
