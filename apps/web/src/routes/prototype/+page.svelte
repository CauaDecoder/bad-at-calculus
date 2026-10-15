<!-- PROTOTYPE — index of the design prototypes (docs/references/design/README.md). -->
<script lang="ts">
  const screens = [
    {
      href: '/prototype/inicio',
      title: 'Início (app)',
      chosen: 'A',
      variants: [
        { key: 'A', label: 'Para hoje' },
        { key: 'B', label: 'Prompt primeiro' },
        { key: 'C', label: 'Painel de progresso' },
      ],
    },
    {
      href: '/prototype/exercicio',
      title: 'Exercício',
      chosen: 'D',
      variants: [
        { key: 'D', label: 'A + barra de progresso da B' },
        { key: 'A', label: 'Laboratório dividido' },
        { key: 'B', label: 'Cartão em foco' },
        { key: 'C', label: 'Caderno' },
      ],
    },
    {
      href: '/prototype/site',
      title: 'Site',
      chosen: 'D',
      variants: [
        { key: 'D', label: 'A + botões da B + scroll da C' },
        { key: 'A', label: 'Problema → prova → CTA' },
        { key: 'B', label: 'Ferramenta no herói' },
        { key: 'C', label: 'Narrativa por scroll' },
      ],
    },
  ];
</script>

<svelte:head><title>Protótipos — Vértice</title></svelte:head>

<main>
  <p class="tag">Protótipo descartável</p>
  <h1>Vértice — protótipos de design</h1>
  <p class="lead">
    Três telas; a escolhida de cada uma está marcada. Use a barra inferior ou <kbd>←</kbd>
    <kbd>→</kbd>
    para trocar. Direção e critérios em <code>docs/references/design/README.md</code>.
  </p>
  <div class="grid">
    {#each screens as screen (screen.href)}
      <section>
        <h2>{screen.title}</h2>
        {#each screen.variants as v (v.key)}
          <a href={`${screen.href}?variant=${v.key}`} class:chosen={v.key === screen.chosen}
            >{v.key} · {v.label}{v.key === screen.chosen ? ' ✓ escolhida' : ''}</a
          >
        {/each}
      </section>
    {/each}
  </div>
</main>

<style>
  main {
    max-width: 960px;
    margin: 0 auto;
    padding: 64px 24px;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: #18181b;
  }

  .tag {
    display: inline-block;
    margin: 0;
    padding: 4px 10px;
    font-size: 12px;
    font-weight: 600;
    color: #b45309;
    background: #fef3c7;
    border-radius: 999px;
  }

  h1 {
    margin: 12px 0 8px;
    font-size: 32px;
    letter-spacing: -0.02em;
  }

  .lead {
    color: #6b6b76;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 16px;
    margin-top: 32px;
  }

  section {
    display: grid;
    gap: 6px;
    padding: 20px;
    border: 1px solid #e7e7ea;
    border-radius: 14px;
  }

  h2 {
    margin: 0 0 8px;
    font-size: 17px;
  }

  a {
    padding: 8px 10px;
    color: #4f46e5;
    text-decoration: none;
    border-radius: 8px;
  }

  a.chosen {
    font-weight: 600;
    background: #eef2ff;
  }

  a:hover {
    background: #eef2ff;
  }

  kbd,
  code {
    font:
      12px 'JetBrains Mono Variable',
      monospace;
  }
</style>
