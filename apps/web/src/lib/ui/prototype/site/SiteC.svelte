<!--
  PROTOTYPE · Site C — "Narrativa por scroll" (dark expressive; Exebenus / Arqitel / Cula).
  A sticky scene of dots morphs as the story scrolls: scattered → f(x) → f′(x) → a check mark
  (a nod to Arqitel's dot-morphing transition). Text sections carry the argument.
-->
<script lang="ts">
  import { resolve } from '$app/paths';
  import { onMount } from 'svelte';

  const N = 48;
  const W = 400;
  const H = 400;

  type Point = { x: number; y: number };

  function curve(fn: (x: number) => number, yMin: number, yMax: number): Point[] {
    return Array.from({ length: N }, (_, i) => {
      const x = -2.4 + (4.8 * i) / (N - 1);
      return { x: ((x + 2.6) / 5.2) * W, y: H - ((fn(x) - yMin) / (yMax - yMin)) * H };
    });
  }

  const scatter: Point[] = Array.from({ length: N }, (_, i) => ({
    x: 40 + ((i * 97) % 320),
    y: 40 + ((i * 53 + (i % 7) * 31) % 320),
  }));

  const check: Point[] = Array.from({ length: N }, (_, i) => {
    const t = i / (N - 1);
    return t < 0.35
      ? { x: 90 + (t / 0.35) * 80, y: 210 + (t / 0.35) * 80 }
      : { x: 170 + ((t - 0.35) / 0.65) * 150, y: 290 - ((t - 0.35) / 0.65) * 190 };
  });

  const stages = [
    scatter,
    curve((x) => x ** 3 - 3 * x + 1, -6, 8),
    curve((x) => 3 * x * x - 3, -6, 16),
    check,
  ];

  const chapters = [
    {
      n: '00',
      title: 'Exatas, sem adivinhar.',
      body: 'Uma plataforma de estudos open source para quem tem prova de Cálculo, Física, POO ou Estruturas.',
    },
    {
      n: '01',
      title: 'Você vê.',
      body: 'Cada função vira um gráfico vivo. Arraste o ponto e a tangente se forma, com os passos da derivada.',
    },
    {
      n: '02',
      title: 'Você pratica.',
      body: 'Sua lista, corrigida na hora. O motor entende formas equivalentes e reconhece o erro que você cometeu.',
    },
    {
      n: '03',
      title: 'Você lembra.',
      body: 'Revisão espaçada do que você erra, até o dia da prova. Tudo no seu computador, sem cadastro.',
    },
  ];

  let stage = $state(0);
  let sections: HTMLElement[] = $state([]);

  // The chapter whose center is closest to the middle of the viewport drives the scene.
  function updateStage(): void {
    const middle = window.innerHeight / 2;
    let best = 0;
    let bestDistance = Infinity;
    sections.forEach((section, i) => {
      const box = section.getBoundingClientRect();
      const distance = Math.abs(box.top + box.height / 2 - middle);
      if (distance < bestDistance) {
        best = i;
        bestDistance = distance;
      }
    });
    stage = best;
  }

  onMount(() => {
    updateStage();
    window.addEventListener('scroll', updateStage, { passive: true });
    window.addEventListener('resize', updateStage);
    return () => {
      window.removeEventListener('scroll', updateStage);
      window.removeEventListener('resize', updateStage);
    };
  });

  const points = $derived(stages[stage] ?? scatter);
  const stageLabel = ['ruído', 'f(x) = x³ − 3x + 1', 'f′(x) = 3x² − 3', 'correto'];
</script>

<div class="sc">
  <nav>
    <span class="brand"><span class="mark"></span>vértice</span>
    <span class="spacer"></span>
    <a href="#github">GitHub</a>
    <a class="nav-cta" href={resolve('/prototype/inicio')}>Começar →</a>
  </nav>

  <div class="story">
    <div class="text">
      {#each chapters as ch, i (ch.n)}
        <section bind:this={sections[i]} data-stage={i} class:active={stage === i}>
          <span class="n">{ch.n}</span>
          <h2>{ch.title}</h2>
          <p>{ch.body}</p>
          {#if i === 0}
            <a class="cta" href={resolve('/prototype/exercicio')}>Resolver um exercício</a>
            <span class="trigger">grátis · sem cadastro · offline</span>
          {/if}
        </section>
      {/each}
    </div>

    <div class="scene" aria-hidden="true">
      <div class="frame">
        <svg viewBox={`0 0 ${W} ${H}`}>
          <line x1="0" y1={H / 2} x2={W} y2={H / 2} class="axis" />
          <line x1={W / 2} y1="0" x2={W / 2} y2={H} class="axis" />
          {#each points as p, i (i)}
            <circle
              r={stage === 3 ? 5 : 4}
              class:hot={stage === 2}
              class:ok={stage === 3}
              style:transform={`translate(${p.x}px, ${p.y}px)`}
              style:transition-delay={`${i * 12}ms`}
            />
          {/each}
        </svg>
        <span class="label">{stageLabel[stage]}</span>
      </div>
    </div>
  </div>

  <section class="final">
    <h2>Sua próxima prova começa com um gráfico.</h2>
    <a class="cta" href={resolve('/prototype/inicio')}>Abrir a Vértice</a>
    <span class="trigger">open source · MIT · roda no navegador</span>
  </section>
</div>

<style>
  .sc {
    --bg: #0e0f11;
    --surface: #16181c;
    --border: #25282e;
    --text: #f2f2f3;
    --muted: #8d8f98;
    --accent: #ff671c;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: var(--text);
    background: var(--bg);
  }

  nav {
    position: sticky;
    top: 0;
    z-index: 5;
    display: flex;
    align-items: center;
    gap: 20px;
    padding: 18px 40px;
    background: linear-gradient(var(--bg), transparent);
  }

  .brand {
    display: flex;
    gap: 10px;
    align-items: center;
    font-weight: 600;
    font-size: 18px;
  }

  .mark {
    width: 18px;
    height: 18px;
    background: var(--accent);
    clip-path: polygon(50% 0, 100% 100%, 0 100%);
  }

  .spacer {
    flex: 1;
  }

  nav a {
    font-size: 14px;
    color: var(--muted);
    text-decoration: none;
  }

  .nav-cta {
    padding: 9px 16px;
    color: #0e0f11 !important;
    font-weight: 600;
    background: var(--text);
    border-radius: 999px;
  }

  .story {
    display: grid;
    grid-template-columns: 1fr 1fr;
    max-width: 1280px;
    margin: 0 auto;
    padding: 0 40px;
  }

  .text section {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-height: 90vh;
    opacity: 0.25;
    transition: opacity 400ms;
  }

  .text section.active {
    opacity: 1;
  }

  .n {
    font:
      500 14px 'JetBrains Mono Variable',
      monospace;
    color: var(--accent);
  }

  h2 {
    margin: 12px 0;
    font-size: clamp(44px, 6.5vw, 96px);
    font-weight: 600;
    line-height: 0.98;
    letter-spacing: -0.045em;
  }

  .text p {
    max-width: 34ch;
    font-size: 19px;
    color: var(--muted);
  }

  .cta {
    align-self: flex-start;
    margin-top: 20px;
    padding: 16px 26px;
    font-weight: 600;
    color: #0e0f11;
    text-decoration: none;
    background: var(--accent);
    border-radius: 999px;
  }

  .trigger {
    margin-top: 10px;
    font-size: 13px;
    color: var(--muted);
  }

  .scene {
    position: sticky;
    top: 0;
    display: grid;
    place-items: center;
    height: 100vh;
  }

  .frame {
    position: relative;
    width: min(520px, 90%);
    aspect-ratio: 1;
    background:
      radial-gradient(circle at 1px 1px, #22252b 1px, transparent 0) 0 0 / 20px 20px,
      radial-gradient(circle at 50% 40%, rgb(255 103 28 / 0.12), transparent 60%);
    border: 1px solid var(--border);
    border-radius: 28px;
  }

  svg {
    width: 100%;
    height: 100%;
    overflow: visible;
  }

  .axis {
    stroke: #2a2d34;
  }

  circle {
    fill: var(--text);
    transition:
      transform 900ms cubic-bezier(0.65, 0, 0.35, 1),
      fill 400ms,
      r 400ms;
  }

  circle.hot {
    fill: var(--accent);
  }

  circle.ok {
    fill: #3ddc97;
  }

  .label {
    position: absolute;
    left: 20px;
    bottom: 16px;
    font:
      500 13px 'JetBrains Mono Variable',
      monospace;
    color: var(--muted);
  }

  .final {
    display: grid;
    justify-items: center;
    padding: 120px 24px 160px;
    text-align: center;
  }

  .final h2 {
    max-width: 16ch;
  }

  .final .cta {
    align-self: center;
  }

  @media (max-width: 860px) {
    .story {
      grid-template-columns: 1fr;
      padding: 0 16px;
    }

    .scene {
      top: 60px;
      height: 50vh;
      grid-row: 1;
      z-index: 1;
      background: var(--bg);
    }

    .text section {
      min-height: 70vh;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    circle {
      transition: none;
    }
  }
</style>
