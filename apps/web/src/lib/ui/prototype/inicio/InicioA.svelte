<!--
  PROTOTYPE · Início A — "Para hoje" (light neutral; Remote / Shopify / Tines).
  RCD: actionable list instead of data, one hero action, started-progress checklist,
  exam countdown as the top-left KPI.
-->
<script lang="ts">
  import { resolve } from '$app/paths';
  import { disciplines, exam, recentLists, setupSteps, student, todos } from '../mock';
  import MorphSelect from '../widgets/MorphSelect.svelte';
  import RollingNumber from '../widgets/RollingNumber.svelte';

  const icons = { list: '∫', review: '↻', mistake: '!', lab: '∿' } as const;
  let discipline = $state('Cálculo A');
  const done = setupSteps.filter((s) => s.done).length;
</script>

<div class="a">
  <aside class="side">
    <a class="brand" href={resolve('/prototype/inicio')}><span class="mark">V</span> Vértice</a>
    <button class="search" type="button">Buscar <kbd>Ctrl K</kbd></button>
    <nav>
      <p>Estudar</p>
      <a class="on" href="#inicio">Início</a>
      <a href="#lab">Laboratório</a>
      <a href="#ex">Exercícios</a>
      <a href="#rev">Revisão <span class="badge">3</span></a>
      <a href="#plano">Plano de estudos</a>
      <p>Biblioteca</p>
      <a href="#listas">Minhas listas</a>
      <a href="#importar">Importar lista</a>
    </nav>
    <div class="me">
      <span class="avatar">C</span>
      <span><b>{student.name}</b><small>{student.course}</small></span>
    </div>
  </aside>

  <main>
    <header>
      <div>
        <h1>Boa noite, {student.name}</h1>
        <p>{exam.title} em {exam.daysLeft} dias. Hoje dá para fechar 3 itens.</p>
      </div>
      <MorphSelect label="Disciplina" groups={disciplines} bind:value={discipline} />
    </header>

    <section class="hero-task">
      <div>
        <span class="eyebrow">Continuar de onde parou</span>
        <h2>Derivadas — exemplo · item 8 de 19</h2>
        <p>Regra da cadeia: calcule g′(x) para g(x) = sen(x²).</p>
        <div class="bar"><span style:width="42%"></span></div>
        <small>42% da lista · 11 itens prontos para a prova</small>
      </div>
      <a class="primary" href={resolve('/prototype/exercicio')}>Continuar item 8 →</a>
    </section>

    <div class="cols">
      <section class="card todo">
        <h3>Para hoje</h3>
        {#each todos as todo (todo.title)}
          <article>
            <span class={`ic ${todo.kind}`}>{icons[todo.kind]}</span>
            <div>
              <b>{todo.title}</b>
              <p>{todo.reason}</p>
            </div>
            <span class="when">{todo.when}</span>
            <button type="button">{todo.cta}</button>
          </article>
        {/each}
      </section>

      <div class="right">
        <section class="card exam">
          <span class="eyebrow">Sua próxima prova</span>
          <div class="count">
            <span class="big"><RollingNumber value={exam.daysLeft} /></span>
            <span>dias<br /><small>{exam.dateLabel}</small></span>
          </div>
          <p><b>{exam.title}</b> · {exam.readyItems} de {exam.totalItems} itens prontos</p>
          <div class="bar">
            <span style:width={`${(exam.readyItems / exam.totalItems) * 100}%`}></span>
          </div>
        </section>

        <section class="card setup">
          <div class="setup-head">
            <h3>Primeiros passos</h3>
            <small>{done} de {setupSteps.length}</small>
          </div>
          <div class="bar thin">
            <span style:width={`${(done / setupSteps.length) * 100}%`}></span>
          </div>
          {#each setupSteps as step (step.label)}
            <p class="step" class:done={step.done}>
              <span class="tick">{step.done ? '✓' : ''}</span>{step.label}
            </p>
          {/each}
        </section>
      </div>
    </div>

    <section class="card lists">
      <h3>Minhas listas</h3>
      {#each recentLists as list (list.title)}
        <div class="row">
          <b>{list.title}</b>
          <small>{list.meta}</small>
          <span class="tags"
            >{#each list.tags as t (t)}<i>{t}</i>{/each}</span
          >
          <span class="mini"><span style:width={`${list.progress}%`}></span></span>
        </div>
      {/each}
    </section>
  </main>
</div>

<style>
  .a {
    --bg: #f7f7f8;
    --surface: #fff;
    --border: #e7e7ea;
    --text: #18181b;
    --muted: #6b6b76;
    --accent: #4f46e5;
    --accent-soft: #eef2ff;
    display: grid;
    grid-template-columns: 248px 1fr;
    min-height: 100dvh;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: var(--text);
    background: var(--bg);
  }

  .side {
    position: sticky;
    top: 0;
    display: flex;
    flex-direction: column;
    gap: 14px;
    height: 100dvh;
    padding: 20px 14px;
    background: var(--surface);
    border-right: 1px solid var(--border);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 8px;
    font-weight: 600;
    font-size: 17px;
    color: var(--text);
    text-decoration: none;
  }

  .mark {
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    color: #fff;
    background: var(--accent);
    border-radius: 8px;
    clip-path: polygon(50% 0, 100% 100%, 0 100%);
    font-size: 11px;
    padding-top: 8px;
  }

  .search {
    display: flex;
    justify-content: space-between;
    padding: 8px 10px;
    font: inherit;
    font-size: 13px;
    color: var(--muted);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
  }

  kbd {
    font:
      11px 'JetBrains Mono Variable',
      monospace;
  }

  nav {
    display: grid;
    gap: 2px;
  }

  nav p {
    margin: 12px 8px 4px;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--muted);
  }

  nav a {
    display: flex;
    justify-content: space-between;
    padding: 7px 10px;
    font-size: 14px;
    color: var(--muted);
    text-decoration: none;
    border-radius: 8px;
  }

  nav a.on {
    color: var(--accent);
    font-weight: 500;
    background: var(--accent-soft);
  }

  .badge {
    padding: 1px 7px;
    font-size: 11px;
    color: #fff;
    background: #e11d48;
    border-radius: 999px;
  }

  .me {
    display: flex;
    gap: 10px;
    align-items: center;
    margin-top: auto;
    padding: 10px;
    border-top: 1px solid var(--border);
  }

  .me small {
    display: block;
    font-size: 12px;
    color: var(--muted);
  }

  .avatar {
    display: grid;
    place-items: center;
    width: 32px;
    height: 32px;
    font-weight: 600;
    color: var(--accent);
    background: var(--accent-soft);
    border-radius: 50%;
  }

  main {
    display: grid;
    gap: 20px;
    align-content: start;
    max-width: 1120px;
    padding: 32px 40px 96px;
  }

  header {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 16px;
  }

  h1 {
    margin: 0;
    font-size: 26px;
    font-weight: 600;
    letter-spacing: -0.02em;
  }

  header p {
    margin: 6px 0 0;
    color: var(--muted);
  }

  .card,
  .hero-task {
    padding: 20px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 14px;
  }

  .hero-task {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    border-color: color-mix(in srgb, var(--accent) 30%, var(--border));
    box-shadow: 0 1px 0 color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .hero-task h2 {
    margin: 4px 0;
    font-size: 19px;
  }

  .hero-task p {
    margin: 0 0 12px;
    color: var(--muted);
  }

  .eyebrow {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--accent);
  }

  .bar {
    height: 8px;
    overflow: hidden;
    background: #efeff2;
    border-radius: 999px;
    max-width: 420px;
  }

  .bar span {
    display: block;
    height: 100%;
    background: var(--accent);
    border-radius: inherit;
    animation: fill 900ms cubic-bezier(0.2, 0.9, 0.1, 1) both;
    transform-origin: left;
  }

  .bar.thin {
    height: 5px;
    margin: 8px 0 12px;
  }

  @keyframes fill {
    from {
      transform: scaleX(0);
    }
  }

  small {
    color: var(--muted);
    font-size: 12px;
  }

  .primary {
    flex-shrink: 0;
    padding: 13px 20px;
    font-weight: 600;
    color: #fff;
    text-decoration: none;
    background: var(--accent);
    border-radius: 10px;
  }

  .cols {
    display: grid;
    grid-template-columns: 1fr 320px;
    gap: 20px;
    align-items: start;
  }

  h3 {
    margin: 0 0 8px;
    font-size: 16px;
  }

  .todo article {
    display: grid;
    grid-template-columns: 40px 1fr auto auto;
    gap: 14px;
    align-items: center;
    padding: 14px 0;
    border-top: 1px solid var(--border);
  }

  .todo article p {
    margin: 2px 0 0;
    font-size: 13px;
    color: var(--muted);
  }

  .ic {
    display: grid;
    place-items: center;
    width: 40px;
    height: 40px;
    font-weight: 700;
    border-radius: 10px;
  }

  .ic.list {
    color: #4f46e5;
    background: #eef2ff;
  }

  .ic.review {
    color: #0e7490;
    background: #ecfeff;
  }

  .ic.mistake {
    color: #c2410c;
    background: #fff7ed;
  }

  .ic.lab {
    color: #15803d;
    background: #f0fdf4;
  }

  .when {
    font-size: 12px;
    color: var(--muted);
  }

  .todo button {
    padding: 7px 12px;
    font: inherit;
    font-size: 13px;
    font-weight: 500;
    color: var(--text);
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
  }

  .right {
    display: grid;
    gap: 20px;
  }

  .count {
    display: flex;
    gap: 12px;
    align-items: center;
    margin: 6px 0;
  }

  .big {
    font-size: 56px;
    font-weight: 600;
    letter-spacing: -0.04em;
  }

  .exam p {
    margin: 0 0 10px;
    font-size: 13px;
    color: var(--muted);
  }

  .setup-head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .step {
    display: flex;
    gap: 10px;
    align-items: center;
    margin: 8px 0;
    font-size: 14px;
  }

  .step.done {
    color: var(--muted);
    text-decoration: line-through;
  }

  .tick {
    display: grid;
    place-items: center;
    width: 20px;
    height: 20px;
    font-size: 12px;
    color: #fff;
    border: 1.5px dashed var(--muted);
    border-radius: 50%;
  }

  .done .tick {
    background: #16a34a;
    border: 0;
  }

  .lists .row {
    display: grid;
    grid-template-columns: 1.2fr 1fr 1fr 120px;
    gap: 12px;
    align-items: center;
    padding: 12px 0;
    border-top: 1px solid var(--border);
    font-size: 14px;
  }

  .tags i {
    margin-right: 6px;
    padding: 2px 8px;
    font-size: 12px;
    font-style: normal;
    color: #4338ca;
    background: #eef2ff;
    border-radius: 6px;
  }

  .mini {
    height: 6px;
    overflow: hidden;
    background: #efeff2;
    border-radius: 999px;
  }

  .mini span {
    display: block;
    height: 100%;
    background: var(--accent);
  }

  @media (max-width: 900px) {
    .a {
      grid-template-columns: 1fr;
    }

    .side {
      display: none;
    }

    main {
      padding: 20px 16px 96px;
    }

    .cols,
    .hero-task {
      grid-template-columns: 1fr;
      flex-direction: column;
      align-items: stretch;
    }

    .todo article {
      grid-template-columns: 40px 1fr;
    }

    .when,
    .lists .row .tags {
      display: none;
    }

    .lists .row {
      grid-template-columns: 1fr 80px;
    }
  }
</style>
