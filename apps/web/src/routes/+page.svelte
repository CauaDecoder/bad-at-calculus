<!-- Home: every module and topic as a card. -->
<script lang="ts">
  import Icon from '$lib/ui/components/Icon.svelte';
  import { MODULES } from '$lib/ui/copy/navigation';
</script>

<svelte:head>
  <title>Math View — laboratório visual de matemática e física</title>
</svelte:head>

<div class="home">
  <header class="hero">
    <h1>Math View</h1>
    <p>
      Um laboratório visual para estudar matemática e física: digite, arraste e veja cada conceito
      acontecer — com os passos da resolução.
    </p>
  </header>

  {#each MODULES as module (module.title)}
    <section class="module" aria-labelledby={`module-${module.icon}`}>
      <h2 id={`module-${module.icon}`} class="module-title">
        <Icon name={module.icon} />
        {module.title}
      </h2>
      <div class="topics">
        {#each module.topics as topic (topic.href)}
          <a class="topic" href={topic.href}>
            <span class="topic-title">
              {topic.title}
              <Icon name="chevronRight" size={14} />
            </span>
            <span class="topic-summary">{topic.summary}</span>
          </a>
        {/each}
      </div>
    </section>
  {/each}
</div>

<style>
  .home {
    max-width: 1040px;
    padding: var(--space-8) var(--space-6);
  }

  .hero h1 {
    margin: 0;
    font-size: 2rem;
    letter-spacing: -0.02em;
  }

  .hero p {
    max-width: 60ch;
    margin: var(--space-2) 0 var(--space-8);
    font-size: var(--text-md);
    color: var(--color-text-muted);
  }

  .module {
    margin-bottom: var(--space-8);
  }

  .module-title {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin: 0 0 var(--space-3);
    font-size: var(--text-lg);
  }

  .topics {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(230px, 1fr));
    gap: var(--space-3);
  }

  .topic {
    display: grid;
    gap: var(--space-2);
    padding: var(--space-4);
    color: var(--color-text);
    text-decoration: none;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-sm);
    transition:
      border-color var(--duration-fast) var(--easing-standard),
      transform var(--duration-fast) var(--easing-standard);
  }

  .topic:hover {
    border-color: var(--color-accent);
    transform: translateY(-1px);
  }

  .topic:focus-visible {
    outline: 2px solid var(--color-accent);
  }

  .topic-title {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-weight: 600;
  }

  .topic-summary {
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }
</style>
