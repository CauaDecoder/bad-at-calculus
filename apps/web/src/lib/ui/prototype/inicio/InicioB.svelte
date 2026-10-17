<!--
  PROTOTYPE · Início B — "Prompt primeiro" (warm paper; hilos / Replit / MagicPath).
  RCD: value first (type a function → instant graph), declarative intent in the omnibox,
  pastel KPI tiles that each carry a sentence of context.
-->
<script lang="ts">
  import { resolve } from '$app/paths';
  import { exam, functions, recentLists, student } from '../mock';
  import RollingNumber from '../widgets/RollingNumber.svelte';
  import TangentGraph from '../widgets/TangentGraph.svelte';

  let query = $state('');
  const chips = [
    { label: 'f(x) = x³ − 3x + 1', value: 'x^3 - 3x + 1' },
    { label: 'Derivar sen(x²)', value: 'sin(x^2)' },
    { label: 'Importar lista (PDF)', value: '' },
    { label: 'Revisar o que vence hoje', value: '' },
  ];

  const preview = $derived(
    query.includes('sin')
      ? functions.sinSquare
      : query.includes('x^3') || query.includes('x³')
        ? functions.cubic
        : query.includes('x')
          ? functions.square
          : null,
  );

  const tiles = [
    { tone: 'salmon', label: 'Revisões', value: 3, note: 'vencem hoje' },
    { tone: 'mustard', label: 'Na fila', value: 12, note: 'itens até a prova' },
    { tone: 'pink', label: 'Sequência', value: 5, note: 'dias seguidos estudando' },
    { tone: 'blue', label: 'Prova', value: exam.daysLeft, note: `dias · ${exam.title}` },
  ];

  const today = new Date().toLocaleDateString('pt-BR', {
    weekday: 'long',
    day: 'numeric',
    month: 'long',
  });
</script>

<div class="b">
  <nav class="rail" aria-label="Navegação">
    <span class="logo">V</span>
    <a class="on" href="#inicio" title="Início">⌂</a>
    <a href="#lab" title="Laboratório">∿</a>
    <a href="#ex" title="Exercícios">✎</a>
    <a href="#rev" title="Revisão">↻</a>
    <a href="#listas" title="Listas">▤</a>
  </nav>

  <main>
    <p class="date">{today}</p>
    <h1>Boa noite, {student.name}.</h1>

    <div class="omni">
      <label for="omni" class="sr">O que você quer estudar?</label>
      <input
        id="omni"
        bind:value={query}
        placeholder="Digite uma função, cole um exercício ou pergunte…"
        autocomplete="off"
      />
      <div class="omni-foot">
        <button type="button" class="ghost">＋ Anexar PDF</button>
        <span class="hint">Enter abre no laboratório</span>
        <button type="button" class="go" aria-label="Abrir">↑</button>
      </div>
      {#if preview}
        <div class="preview">
          <TangentGraph f={preview.f} df={preview.df} />
          <div class="preview-side">
            <b>f(x) = {preview.label}</b>
            <p>Arraste o ponto. A tangente e f′ acompanham.</p>
            <a href={resolve('/prototype/exercicio')}>Praticar exercícios com esta função →</a>
          </div>
        </div>
      {/if}
    </div>

    <div class="chips">
      <span>Experimente</span>
      {#each chips as chip (chip.label)}
        <button type="button" onclick={() => (query = chip.value)}>{chip.label}</button>
      {/each}
    </div>

    <section class="tiles">
      {#each tiles as tile (tile.label)}
        <article class={tile.tone}>
          <span class="t-label">{tile.label}</span>
          <span class="t-value"><RollingNumber value={tile.value} /></span>
          <span class="t-note">{tile.note}</span>
        </article>
      {/each}
    </section>

    <section class="recent">
      <h2>Suas listas</h2>
      <div class="grid">
        <a class="new" href="#importar">＋<span>Importar ou criar lista</span></a>
        {#each recentLists as list (list.title)}
          <a class="list" href={resolve('/prototype/exercicio')}>
            <span class="thumb">
              <svg viewBox="0 0 100 40" aria-hidden="true"
                ><path d="M0 30 C 20 5, 40 5, 50 20 S 80 38, 100 8" /></svg
              >
            </span>
            <b>{list.title}</b>
            <small>{list.meta}</small>
          </a>
        {/each}
      </div>
    </section>
  </main>
</div>

<style>
  .b {
    --bg: #f6f4ef;
    --surface: #fffdf9;
    --border: #e8e2d6;
    --text: #1f1b16;
    --muted: #7a7268;
    --accent: #d9541e;
    --serif: 'Iowan Old Style', 'Palatino Linotype', Palatino, Georgia, serif;
    --g-bg: var(--surface);
    --g-grid: #efe9de;
    --g-axis: #cfc5b4;
    --g-curve: var(--text);
    --g-point: var(--accent);
    --g-text: var(--text);
    --g-muted: var(--muted);
    display: grid;
    grid-template-columns: 64px 1fr;
    min-height: 100dvh;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: var(--text);
    background: var(--bg);
  }

  .rail {
    position: sticky;
    top: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    height: 100dvh;
    padding: 16px 0;
    border-right: 1px solid var(--border);
  }

  .logo {
    margin-bottom: 14px;
    font-family: var(--serif);
    font-size: 24px;
    font-weight: 700;
    color: var(--accent);
  }

  .rail a {
    display: grid;
    place-items: center;
    width: 40px;
    height: 40px;
    font-size: 18px;
    color: var(--muted);
    text-decoration: none;
    border-radius: 10px;
  }

  .rail a.on {
    color: var(--text);
    background: #ebe5d9;
  }

  main {
    width: min(880px, 100%);
    margin: 0 auto;
    padding: 72px 24px 120px;
  }

  .date {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--muted);
  }

  h1 {
    margin: 10px 0 28px;
    font-family: var(--serif);
    font-size: clamp(36px, 6vw, 56px);
    font-weight: 600;
    letter-spacing: -0.02em;
    line-height: 1.05;
  }

  .omni {
    padding: 14px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 18px;
    box-shadow: 0 8px 30px rgb(80 60 30 / 0.07);
  }

  .omni input {
    width: 100%;
    padding: 8px 6px 14px;
    font:
      18px 'Inter Variable',
      system-ui,
      sans-serif;
    color: var(--text);
    background: transparent;
    border: 0;
    outline: none;
  }

  .omni-foot {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .ghost {
    padding: 6px 12px;
    font: inherit;
    font-size: 13px;
    color: var(--muted);
    background: transparent;
    border: 1px dashed var(--border);
    border-radius: 999px;
  }

  .hint {
    margin-left: auto;
    font-size: 12px;
    color: var(--muted);
  }

  .go {
    width: 36px;
    height: 36px;
    font-size: 16px;
    color: #fff;
    background: var(--accent);
    border: 0;
    border-radius: 50%;
  }

  .preview {
    display: grid;
    grid-template-columns: 1.4fr 1fr;
    gap: 20px;
    margin-top: 14px;
    padding-top: 14px;
    border-top: 1px solid var(--border);
    animation: rise 320ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  .preview-side p {
    color: var(--muted);
    font-size: 14px;
  }

  .preview-side a {
    font-weight: 600;
    color: var(--accent);
    text-decoration: none;
  }

  @keyframes rise {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
  }

  .chips {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
    margin: 16px 0 40px;
    font-size: 13px;
    color: var(--muted);
  }

  .chips button {
    padding: 7px 12px;
    font: inherit;
    color: var(--text);
    background: #efeadf;
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
  }

  .tiles {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 14px;
  }

  .tiles article {
    display: grid;
    gap: 6px;
    padding: 16px;
    border-radius: 16px;
    border: 1px solid rgb(0 0 0 / 0.06);
  }

  .salmon {
    background: #f6b9a3;
  }

  .mustard {
    background: #f3d98b;
  }

  .pink {
    background: #f1c6da;
  }

  .blue {
    background: #b9d4f1;
  }

  .t-label {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    text-align: right;
  }

  .t-value {
    font:
      600 42px/1 'JetBrains Mono Variable',
      monospace;
  }

  .t-note {
    font-size: 13px;
  }

  .recent h2 {
    margin: 44px 0 14px;
    font-family: var(--serif);
    font-size: 24px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
    gap: 14px;
  }

  .grid a {
    display: grid;
    gap: 4px;
    padding: 12px;
    color: var(--text);
    text-decoration: none;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 14px;
  }

  .new {
    place-content: center;
    min-height: 150px;
    font-size: 26px;
    text-align: center;
    color: var(--accent) !important;
    border-style: dashed !important;
  }

  .new span {
    font-size: 13px;
    font-weight: 600;
  }

  .thumb {
    height: 70px;
    margin-bottom: 6px;
    background: #f3eee4;
    border-radius: 10px;
  }

  .thumb svg {
    width: 100%;
    height: 100%;
    fill: none;
    stroke: var(--accent);
    stroke-width: 2;
  }

  small {
    font-size: 12px;
    color: var(--muted);
  }

  .sr {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip: rect(0 0 0 0);
  }

  @media (max-width: 760px) {
    .b {
      grid-template-columns: 1fr;
    }

    .rail {
      display: none;
    }

    main {
      padding-top: 32px;
    }

    .tiles,
    .preview {
      grid-template-columns: 1fr 1fr;
    }

    .preview {
      grid-template-columns: 1fr;
    }
  }
</style>
