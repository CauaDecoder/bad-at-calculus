<!--
  PROTOTYPE · Site D — chosen mix:
  - layout, copy and palette of variant A (problem → proof → features → FAQ → CTA);
  - in the hero demo, the three function buttons of variant B switch the graph;
  - the *idea* of variant C (things move with the scroll), adapted to A's own pieces instead of
    a separate scene: each feature screenshot is driven by how far it has scrolled —
    the tangent sweeps the curve, answers type themselves and get corrected, review cards
    line up by due date. Sections also reveal on scroll and the hero demo has a light parallax.
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import { functions } from '../mock';
  import TangentGraph from '../widgets/TangentGraph.svelte';
  import { reveal } from '../widgets/reveal';
  import { scrollProgress, segment } from '../widgets/scroll-progress';

  const heroFunctions = [
    {
      key: 'cubic',
      input: 'x^3 - 3x + 1',
      question: 'Em quais x a tangente a f(x) = x³ − 3x + 1 é horizontal?',
      answer: '{-1, 1}',
      derivative: 'f′(x) = 3x² − 3',
    },
    {
      key: 'square',
      input: 'x^2',
      question: 'Qual a inclinação da tangente a f(x) = x² em x = 3?',
      answer: '6',
      derivative: 'f′(x) = 2x',
    },
    {
      key: 'sinSquare',
      input: 'sin(x^2)',
      question: 'Derive g(x) = sen(x²).',
      answer: '2x·cos(x²)',
      derivative: 'g′(x) = 2x·cos(x²)',
    },
  ] as const;

  let selected = $state(0);
  const current = $derived(heroFunctions[selected] as (typeof heroFunctions)[number]);
  const fn = $derived(functions[current.key]);

  const pains = [
    {
      title: 'O gabarito só diz "errado"',
      body: 'A resposta no fim do PDF não mostra em que passo você se perdeu.',
    },
    {
      title: 'Ferramentas que não conversam',
      body: 'Calculadora gráfica num lugar, CAS em outro, flashcards em um terceiro.',
    },
    {
      title: 'O mesmo erro na prova',
      body: 'Sem registro do que você erra, a regra da cadeia volta a te pegar.',
    },
  ];

  const features = [
    {
      tag: 'Laboratório',
      title: 'Veja o conceito acontecer',
      body: 'Arraste o ponto e a tangente, a derivada e a área se formam na tela, com os passos.',
    },
    {
      tag: 'Exercícios',
      title: 'Correção na hora, sem decorar formato',
      body: 'y = 5x − 16 ou 5x − y = 16: o motor entende que é a mesma reta e aponta o erro comum.',
    },
    {
      tag: 'Revisão',
      title: 'Revise só o que você erra',
      body: 'Repetição espaçada (FSRS) e um caderno de erros automático até o dia da prova.',
    },
  ];

  // Scroll progress (0..1) of each feature screenshot.
  let progress = $state([0, 0, 0]);
  const setProgress = (i: number) => (p: number) => {
    progress[i] = p;
  };

  // Feature 1 — the tangent point travels along the curve as you scroll.
  const labX0 = $derived(-2.2 + 4.4 * (progress[0] ?? 0));

  // Feature 2 — two answers type themselves, then the engine's verdict appears.
  const answers = [
    { text: '5x − y = 16', verdict: '✓ mesma reta', ok: true, from: 0.05, to: 0.45 },
    { text: 'cos(x²)', verdict: 'faltou × 2x', ok: false, from: 0.5, to: 0.85 },
  ];
  const typed = (i: number) => {
    const a = answers[i];
    if (!a) return { shown: '', done: false };
    const t = segment(progress[1] ?? 0, a.from, a.to);
    return { shown: a.text.slice(0, Math.round(t * a.text.length)), done: t >= 1 };
  };

  // Feature 3 — review cards slide into place one after another, ordered by due date.
  const reviews = [
    { topic: 'Regra da cadeia', due: 'hoje', urgent: true },
    { topic: 'Limites laterais', due: 'em 3 dias', urgent: false },
    { topic: 'Reta tangente', due: 'em 6 dias', urgent: false },
  ];
  const cardIn = (i: number) => segment(progress[2] ?? 0, i * 0.22, i * 0.22 + 0.45);

  const faq = [
    { q: 'É de graça mesmo?', a: 'Sim. É open source (MIT/Apache) e roda no seu navegador.' },
    {
      q: 'Preciso criar conta?',
      a: 'Não. Tudo fica salvo no seu dispositivo. Contas só para sincronizar, no futuro.',
    },
    {
      q: 'Funciona com a lista da minha turma?',
      a: 'Sim. Importe em lote (PDF, planilha ou pack) e a lista fica privada, só com você.',
    },
    {
      q: 'Quais matérias?',
      a: 'Cálculo e Vetores hoje. Física I, POO, Algoritmos e Estruturas (AU) estão a caminho.',
    },
    {
      q: 'A IA resolve por mim?',
      a: 'Não. O motor corrige e a IA só dá dicas, um passo de cada vez.',
    },
  ];

  let parallax = $state(0);

  function onScroll(): void {
    parallax = Math.min(60, window.scrollY * 0.08);
  }

  onMount(() => {
    onScroll();
    window.addEventListener('scroll', onScroll, { passive: true });
    window.addEventListener('resize', onScroll);
    return () => {
      window.removeEventListener('scroll', onScroll);
      window.removeEventListener('resize', onScroll);
    };
  });
</script>

<div class="sd">
  <nav>
    <span class="brand"><span class="mark"></span>Vértice</span>
    <a href="#como">Como funciona</a>
    <a href="#materias">Matérias</a>
    <a href="#faq">Perguntas</a>
    <a class="gh" href="#github">★ GitHub</a>
    <a class="nav-cta" href="/prototype/inicio?variant=A">Abrir o app</a>
  </nav>

  <header class="hero">
    <div class="copy">
      <p class="pill">Open source · Cálculo, Vetores, Física, POO, Estruturas</p>
      <h1>O gabarito diz que você errou. <em>A Vértice mostra onde.</em></h1>
      <p class="sub">
        Resolva suas listas de exatas com correção na hora, veja cada passo no gráfico e revise só o
        que você erra, até o dia da prova.
      </p>
      <div class="ctas">
        <a class="primary" href="/prototype/exercicio?variant=D">Resolver um exercício agora</a>
        <a class="secondary" href="/prototype/inicio?variant=A">Abrir o laboratório</a>
      </div>
      <p class="trigger">Grátis · sem cadastro · funciona offline</p>
    </div>

    <div class="demo" style:transform={`translateY(${-parallax}px)`}>
      <div class="demo-bar"><span></span><span></span><span></span> Laboratório · experimente</div>
      <div class="fn-row" role="tablist" aria-label="Função do gráfico">
        <span class="fx">f(x) =</span>
        {#each heroFunctions as option, i (option.key)}
          <button
            type="button"
            role="tab"
            aria-selected={i === selected}
            class:on={i === selected}
            onclick={() => (selected = i)}>{option.input}</button
          >
        {/each}
      </div>
      <div class="demo-body">
        {#key selected}
          <p class="question">{current.question}</p>
          <div class="fake-input">{current.answer} <span class="ok">✓ Correto</span></div>
        {/key}
        <TangentGraph f={fn.f} df={fn.df} readout={false} />
        <p class="deriv-text">{current.derivative}</p>
      </div>
    </div>
  </header>

  <section class="proof reveal" use:reveal>
    <span><b>7</b> tópicos prontos</span>
    <span><b>0</b> cadastros</span>
    <span>motor em <b>Rust</b> no navegador</span>
    <span>licença <b>MIT</b></span>
  </section>

  <section class="pains">
    <h2 class="reveal" use:reveal>Estudar exatas não deveria ser adivinhar onde errou</h2>
    <div class="three">
      {#each pains as p, i (p.title)}
        <article class="reveal" use:reveal style:transition-delay={`${i * 90}ms`}>
          <h3>{p.title}</h3>
          <p>{p.body}</p>
        </article>
      {/each}
    </div>
  </section>

  <section class="features" id="como">
    {#each features as f, i (f.title)}
      <article class:flip={i % 2 === 1}>
        <div class="reveal" use:reveal>
          <span class="tag">{f.tag}</span>
          <h3>{f.title}</h3>
          <p>{f.body}</p>
        </div>
        <div class="shot" use:scrollProgress={setProgress(i)}>
          {#if i === 0}
            <TangentGraph
              f={functions.cubic.f}
              df={functions.cubic.df}
              x0={labX0}
              autoplay={false}
            />
            <p class="hint">
              Role a página: a tangente percorre a curva e fica horizontal em x = ±1.
            </p>
          {:else if i === 1}
            {#each answers as a, j (a.text)}
              {@const t = typed(j)}
              <div class="fake-input big typing" class:wrong={!a.ok} class:done={t.done}>
                <span class="typed">{t.shown}<span class="caret" class:hide={t.done}></span></span>
                {#if t.done}<span class={a.ok ? 'ok' : 'bad'}>{a.verdict}</span>{/if}
              </div>
            {/each}
          {:else}
            <div class="cards">
              {#each reviews as r, j (r.topic)}
                {@const k = cardIn(j)}
                <div
                  class="review"
                  class:urgent={r.urgent && k >= 1}
                  style:opacity={k}
                  style:transform={`translateX(${(1 - k) * 56}px)`}
                >
                  <span>{r.topic}</span>
                  <span class="due">{r.due}</span>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </article>
    {/each}
  </section>

  <section class="faq reveal" id="faq" use:reveal>
    <h2>Perguntas</h2>
    {#each faq as item (item.q)}
      <details>
        <summary>{item.q}</summary>
        <p>{item.a}</p>
      </details>
    {/each}
  </section>

  <section class="final reveal" use:reveal>
    <h2>Sua próxima lista, corrigida na hora.</h2>
    <a class="primary" href="/prototype/exercicio?variant=D">Resolver um exercício agora</a>
    <p class="trigger">Grátis · sem cadastro · funciona offline</p>
  </section>
</div>

<style>
  .sd {
    --bg: #ffffff;
    --soft: #f7f7f8;
    --border: #e7e7ea;
    --text: #18181b;
    --muted: #6b6b76;
    --accent: #4f46e5;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: var(--text);
    background: var(--bg);
  }

  /* ── Scroll reveal (from variant C's motion) ─────────────────────── */
  .reveal {
    transition:
      opacity 700ms cubic-bezier(0.2, 0.9, 0.1, 1),
      transform 700ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  .reveal:not(:global(.is-visible)) {
    opacity: 0;
    transform: translateY(28px);
  }

  nav {
    position: sticky;
    top: 0;
    z-index: 5;
    display: flex;
    align-items: center;
    gap: 24px;
    padding: 14px 48px;
    background: rgb(255 255 255 / 0.85);
    backdrop-filter: blur(10px);
    border-bottom: 1px solid var(--border);
  }

  nav a {
    font-size: 14px;
    color: var(--muted);
    text-decoration: none;
  }

  .brand {
    display: flex;
    gap: 8px;
    align-items: center;
    margin-right: auto;
    font-weight: 600;
    font-size: 17px;
  }

  .mark {
    width: 18px;
    height: 18px;
    background: var(--accent);
    clip-path: polygon(50% 0, 100% 100%, 0 100%);
  }

  .nav-cta {
    padding: 8px 14px;
    font-weight: 600;
    color: #fff !important;
    background: var(--text);
    border-radius: 8px;
  }

  .hero {
    display: grid;
    grid-template-columns: 1.05fr 1fr;
    gap: 48px;
    align-items: center;
    max-width: 1200px;
    margin: 0 auto;
    padding: 72px 48px 48px;
  }

  .pill {
    display: inline-block;
    margin: 0;
    padding: 6px 12px;
    font-size: 13px;
    color: var(--accent);
    background: #eef2ff;
    border-radius: 999px;
  }

  h1 {
    margin: 18px 0;
    font-size: clamp(36px, 5vw, 58px);
    line-height: 1.05;
    letter-spacing: -0.035em;
  }

  h1 em {
    font-style: normal;
    color: var(--accent);
  }

  .sub {
    max-width: 52ch;
    font-size: 18px;
    color: var(--muted);
  }

  .ctas {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    margin-top: 24px;
  }

  .primary {
    padding: 16px 24px;
    font-weight: 600;
    color: #fff;
    text-decoration: none;
    background: var(--accent);
    border-radius: 12px;
    box-shadow: 0 10px 24px rgb(79 70 229 / 0.3);
  }

  .secondary {
    padding: 16px 20px;
    font-weight: 600;
    color: var(--text);
    text-decoration: none;
    border: 1px solid var(--border);
    border-radius: 12px;
  }

  .trigger {
    margin: 10px 0 0;
    font-size: 13px;
    color: var(--muted);
  }

  .demo {
    overflow: hidden;
    background: #fff;
    border: 1px solid var(--border);
    border-radius: 18px;
    box-shadow: 0 30px 80px rgb(24 24 27 / 0.12);
    will-change: transform;
  }

  .demo-bar {
    display: flex;
    gap: 6px;
    align-items: center;
    padding: 12px 16px;
    font-size: 12px;
    color: var(--muted);
    background: var(--soft);
    border-bottom: 1px solid var(--border);
  }

  .demo-bar span {
    width: 10px;
    height: 10px;
    background: #d4d4d8;
    border-radius: 50%;
  }

  /* Function buttons from variant B, recolored to the light-neutral palette. */
  .fn-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
    padding: 14px 20px 0;
  }

  .fx {
    margin-right: 2px;
    font:
      600 15px 'JetBrains Mono Variable',
      monospace;
  }

  .fn-row button {
    padding: 7px 12px;
    font:
      500 14px 'JetBrains Mono Variable',
      monospace;
    color: var(--muted);
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 10px;
    cursor: pointer;
    transition:
      background 200ms,
      color 200ms,
      border-color 200ms;
  }

  .fn-row button:hover {
    border-color: #c7d2fe;
  }

  .fn-row button.on {
    color: #fff;
    background: var(--accent);
    border-color: var(--accent);
  }

  .demo-body {
    display: grid;
    gap: 12px;
    padding: 16px 20px 20px;
  }

  .question {
    margin: 0;
    font-weight: 500;
    animation: swap 320ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  .demo-body .fake-input {
    animation: swap 320ms 60ms cubic-bezier(0.2, 0.9, 0.1, 1) both;
  }

  @keyframes swap {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
  }

  .deriv-text {
    margin: 0;
    font:
      500 13px 'JetBrains Mono Variable',
      monospace;
    color: var(--muted);
  }

  .fake-input {
    display: flex;
    justify-content: space-between;
    padding: 12px 14px;
    font:
      500 15px 'JetBrains Mono Variable',
      monospace;
    border: 1.5px solid #bbf7d0;
    border-radius: 10px;
  }

  .fake-input.big {
    margin-top: 12px;
    font-size: 17px;
    background: #fff;
  }

  .fake-input.wrong {
    border-color: #fed7aa;
  }

  .fake-input.wrong span {
    color: #c2410c;
    font-family: 'Inter Variable', sans-serif;
    font-size: 13px;
  }

  .ok {
    color: #16a34a;
    font-family: 'Inter Variable', sans-serif;
    font-size: 13px;
  }

  .proof {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 40px;
    padding: 22px;
    font-size: 15px;
    color: var(--muted);
    background: var(--soft);
    border-block: 1px solid var(--border);
  }

  .proof b {
    color: var(--text);
  }

  section h2 {
    margin: 0 0 28px;
    font-size: 34px;
    letter-spacing: -0.025em;
    text-align: center;
  }

  .pains,
  .faq,
  .final {
    max-width: 1100px;
    margin: 0 auto;
    padding: 80px 48px 0;
  }

  .three {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
  }

  .three article {
    padding: 24px;
    background: var(--soft);
    border-radius: 16px;
  }

  .three h3 {
    margin: 0 0 8px;
    font-size: 18px;
  }

  .three p,
  .features p {
    margin: 0;
    color: var(--muted);
  }

  /* ── Features (layout of variant A) with scroll-linked screenshots ── */
  .features {
    max-width: 1100px;
    margin: 0 auto;
    padding: 80px 48px 0;
  }

  .features article {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 48px;
    align-items: center;
    padding: 36px 0;
  }

  .features article.flip > :first-child {
    order: 2;
  }

  .tag {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--accent);
  }

  .features h3 {
    margin: 8px 0;
    font-size: 26px;
    letter-spacing: -0.02em;
  }

  .shot {
    display: grid;
    gap: 12px;
    padding: 20px;
    background: var(--soft);
    border: 1px solid var(--border);
    border-radius: 16px;
  }

  .shot .hint {
    font-size: 12px;
  }

  .typing {
    align-items: center;
    min-height: 52px;
    margin: 0;
    background: #fff;
    border-color: var(--border);
    transition: border-color 240ms;
  }

  .typing.done {
    border-color: #bbf7d0;
  }

  .typing.wrong.done {
    border-color: #fed7aa;
  }

  .caret {
    display: inline-block;
    width: 2px;
    height: 1.1em;
    margin-left: 2px;
    vertical-align: text-bottom;
    background: var(--accent);
    animation: blink 1s steps(2) infinite;
  }

  .caret.hide {
    display: none;
  }

  @keyframes blink {
    50% {
      opacity: 0;
    }
  }

  .typing .ok,
  .typing .bad {
    font-family: 'Inter Variable', sans-serif;
    font-size: 13px;
    animation: pop 320ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  .typing .bad {
    color: #c2410c;
  }

  @keyframes pop {
    from {
      opacity: 0;
      transform: scale(0.85);
    }
  }

  .cards {
    display: grid;
    gap: 10px;
  }

  .review {
    display: flex;
    justify-content: space-between;
    padding: 14px;
    background: #fff;
    border: 1px solid var(--border);
    border-radius: 10px;
    will-change: transform, opacity;
  }

  .review .due {
    font-size: 13px;
    color: var(--muted);
  }

  .review.urgent {
    border-color: #c7d2fe;
    box-shadow: 0 0 0 3px #eef2ff;
  }

  .review.urgent .due {
    font-weight: 600;
    color: var(--accent);
  }

  details {
    max-width: 720px;
    margin: 0 auto;
    padding: 18px 0;
    border-bottom: 1px solid var(--border);
  }

  summary {
    font-weight: 600;
    cursor: pointer;
  }

  details p {
    color: var(--muted);
  }

  .final {
    padding-bottom: 140px;
    text-align: center;
  }

  .final .primary {
    display: inline-block;
  }

  @media (max-width: 900px) {
    nav a:not(.nav-cta) {
      display: none;
    }

    nav {
      padding: 12px 16px;
    }

    .hero,
    .three,
    .features article {
      grid-template-columns: 1fr;
    }

    .hero,
    .pains,
    .features,
    .faq,
    .final {
      padding-inline: 16px;
    }

    .features article.flip > :first-child {
      order: 0;
    }

    .demo {
      transform: none !important;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .reveal,
    .caret {
      transition: none;
      animation: none;
    }

    .demo {
      transform: none !important;
    }
  }
</style>
