<!--
  PROTOTYPE · Início C — "Painel de progresso" (dark expressive; Exebenus / Heron AI).
  RCD: main KPI top-left with meaning attached, charts that morph instead of redraw, and an
  "o que fazer agora" column so the data always ends in an action.
-->
<script lang="ts">
  import { accuracy, exam, functions, masteryByTopic, student, type RangeKey } from '../mock';
  import Donut from '../widgets/Donut.svelte';
  import LineChart from '../widgets/LineChart.svelte';
  import RollingNumber from '../widgets/RollingNumber.svelte';
  import TangentGraph from '../widgets/TangentGraph.svelte';

  let range: RangeKey = $state('30d');
  let period: 'semana' | 'mes' = $state('semana');
  const readiness = $derived(period === 'semana' ? 58 : 41);

  const actions = [
    {
      title: 'Derivadas está em 41%',
      body: 'É o tópico que mais pesa na P1. Resolva 3 itens de regra da cadeia.',
      cta: 'Treinar 3 itens',
    },
    {
      title: '3 revisões vencem hoje',
      body: 'Revisar agora mantém Limites acima de 80% até a prova.',
      cta: 'Revisar',
    },
    {
      title: 'Você acerta mais de manhã',
      body: '74% de acertos antes do meio-dia contra 51% à noite.',
      cta: 'Agendar sessão',
    },
  ];
</script>

<div class="c">
  <header class="top">
    <span class="brand"><span class="mark"></span>vértice</span>
    <nav>
      <a class="on" href="#painel">Painel</a>
      <a href="#lab">Laboratório</a>
      <a href="#ex">Exercícios</a>
      <a href="#rev">Revisão</a>
    </nav>
    <span class="who">{student.name}</span>
  </header>

  <main>
    <section class="kpi">
      <p class="eyebrow">Prontidão para a {exam.title}</p>
      <div class="kpi-row">
        <span class="huge"><RollingNumber value={`${readiness}%`} /></span>
        <p>
          Faltam <b>{exam.daysLeft} dias</b>. Com 3 itens por dia você chega a <b>86%</b> na véspera.
        </p>
      </div>
      <a class="cta" href="/prototype/exercicio?variant=C">Estudar agora — 25 min</a>
    </section>

    <section class="card donut-card">
      <div class="head">
        <h2>Domínio por tópico</h2>
        <div class="seg" role="tablist" aria-label="Período">
          <button
            type="button"
            role="tab"
            aria-selected={period === 'semana'}
            onclick={() => (period = 'semana')}>Semana</button
          >
          <button
            type="button"
            role="tab"
            aria-selected={period === 'mes'}
            onclick={() => (period = 'mes')}>Mês</button
          >
        </div>
      </div>
      <div class="donut-row">
        <Donut segments={masteryByTopic[period]} />
        <ul>
          {#each masteryByTopic[period] as m (m.topic)}
            <li><span class="sw" style:background={m.color}></span>{m.topic}<b>{m.value}%</b></li>
          {/each}
        </ul>
      </div>
    </section>

    <section class="card line-card">
      <div class="head">
        <h2>Acertos por dia</h2>
        <div class="seg" role="tablist" aria-label="Intervalo">
          {#each ['7d', '30d', '90d'] as const as r (r)}
            <button type="button" role="tab" aria-selected={range === r} onclick={() => (range = r)}
              >{r.replace('d', ' dias')}</button
            >
          {/each}
        </div>
      </div>
      <LineChart
        label="Acertos por dia"
        current={accuracy[range].current}
        previous={accuracy[range].previous}
      />
      <p class="meaning">+18% em relação ao período anterior. Você está no ritmo certo.</p>
    </section>

    <section class="actions">
      <h2>O que fazer agora</h2>
      {#each actions as action, i (action.title)}
        <article style:animation-delay={`${i * 80}ms`}>
          <b>{action.title}</b>
          <p>{action.body}</p>
          <button type="button">{action.cta} →</button>
        </article>
      {/each}
    </section>

    <section class="card lab">
      <h2>Laboratório do dia</h2>
      <TangentGraph f={functions.cubic.f} df={functions.cubic.df} />
    </section>
  </main>
</div>

<style>
  .c {
    --bg: #0e0f11;
    --surface: #16181c;
    --border: #25282e;
    --text: #f2f2f3;
    --muted: #8d8f98;
    --accent: #ff671c;
    --p-c1: #ff671c;
    --p-c2: #3d5afe;
    --p-c3: #3ddc97;
    --p-c4: #f5c84b;
    --d-track: #22252b;
    --d-text: var(--text);
    --d-muted: var(--muted);
    --lc-color: var(--accent);
    --lc-muted: #5c5f68;
    --lc-text: var(--text);
    --lc-bg: var(--surface);
    --g-bg: var(--surface);
    --g-grid: #1f2228;
    --g-axis: #3a3e46;
    --g-curve: var(--text);
    --g-tangent: #3ddc97;
    --g-deriv: #ff671c;
    --g-point: #3d5afe;
    --g-text: var(--text);
    --g-muted: var(--muted);
    min-height: 100dvh;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: var(--text);
    background:
      radial-gradient(circle at 1px 1px, #1b1d22 1px, transparent 0) 0 0 / 22px 22px,
      var(--bg);
  }

  .top {
    display: flex;
    align-items: center;
    gap: 32px;
    padding: 18px 32px;
    border-bottom: 1px solid var(--border);
  }

  .brand {
    display: flex;
    gap: 10px;
    align-items: center;
    font-weight: 600;
    font-size: 18px;
    letter-spacing: -0.02em;
  }

  .mark {
    width: 18px;
    height: 18px;
    background: var(--accent);
    clip-path: polygon(50% 0, 100% 100%, 0 100%);
  }

  .top nav {
    display: flex;
    gap: 4px;
  }

  .top nav a {
    padding: 8px 12px;
    font-size: 14px;
    color: var(--muted);
    text-decoration: none;
    border-radius: 8px;
  }

  .top nav a.on {
    color: var(--text);
    background: var(--surface);
  }

  .who {
    margin-left: auto;
    color: var(--muted);
    font-size: 14px;
  }

  main {
    display: grid;
    grid-template-columns: 1.15fr 1fr;
    grid-template-areas:
      'kpi actions'
      'donut actions'
      'line lab';
    gap: 18px;
    max-width: 1240px;
    margin: 0 auto;
    padding: 32px 32px 110px;
  }

  .kpi {
    grid-area: kpi;
    padding: 8px 0 12px;
  }

  .donut-card {
    grid-area: donut;
  }

  .line-card {
    grid-area: line;
  }

  .actions {
    grid-area: actions;
    display: grid;
    gap: 12px;
    align-content: start;
  }

  .lab {
    grid-area: lab;
  }

  .eyebrow {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--accent);
  }

  .kpi-row {
    display: flex;
    gap: 20px;
    align-items: flex-end;
  }

  .huge {
    font-size: clamp(64px, 9vw, 112px);
    font-weight: 600;
    letter-spacing: -0.05em;
    line-height: 1;
  }

  .kpi-row p {
    max-width: 30ch;
    margin: 0 0 12px;
    color: var(--muted);
  }

  .kpi-row b {
    color: var(--text);
  }

  .cta {
    display: inline-block;
    margin-top: 16px;
    padding: 14px 22px;
    font-weight: 600;
    color: #0e0f11;
    text-decoration: none;
    background: var(--accent);
    border-radius: 999px;
    box-shadow: 0 0 0 0 rgb(255 103 28 / 0.5);
    animation: pulse 2.4s infinite;
  }

  @keyframes pulse {
    70% {
      box-shadow: 0 0 0 14px rgb(255 103 28 / 0);
    }
  }

  .card,
  .actions article {
    padding: 20px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 16px;
  }

  h2 {
    margin: 0;
    font-size: 15px;
    font-weight: 600;
  }

  .head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .seg {
    display: flex;
    padding: 3px;
    background: #0e0f11;
    border: 1px solid var(--border);
    border-radius: 999px;
  }

  .seg button {
    padding: 6px 12px;
    font: inherit;
    font-size: 12px;
    color: var(--muted);
    background: transparent;
    border: 0;
    border-radius: 999px;
    cursor: pointer;
    transition:
      background 200ms,
      color 200ms;
  }

  .seg button[aria-selected='true'] {
    color: var(--text);
    background: #2a2d34;
  }

  .donut-row {
    display: flex;
    gap: 28px;
    align-items: center;
  }

  .donut-row ul {
    display: grid;
    gap: 10px;
    flex: 1;
    margin: 0;
    padding: 0;
    list-style: none;
    font-size: 14px;
  }

  .donut-row li {
    display: flex;
    gap: 10px;
    align-items: center;
  }

  .donut-row li b {
    margin-left: auto;
    font-variant-numeric: tabular-nums;
  }

  .sw {
    width: 10px;
    height: 10px;
    border-radius: 3px;
  }

  .meaning {
    margin: 12px 0 0;
    font-size: 13px;
    color: #3ddc97;
  }

  .actions article {
    animation: rise 420ms cubic-bezier(0.2, 0.9, 0.1, 1) both;
  }

  .actions article p {
    margin: 6px 0 12px;
    font-size: 14px;
    color: var(--muted);
  }

  .actions button {
    padding: 0;
    font: inherit;
    font-weight: 600;
    color: var(--accent);
    background: none;
    border: 0;
    cursor: pointer;
  }

  @keyframes rise {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
  }

  @media (max-width: 980px) {
    main {
      grid-template-columns: 1fr;
      grid-template-areas: 'kpi' 'actions' 'donut' 'line' 'lab';
      padding: 20px 16px 110px;
    }

    .top nav {
      display: none;
    }

    .donut-row {
      flex-direction: column;
      align-items: stretch;
    }
  }
</style>
