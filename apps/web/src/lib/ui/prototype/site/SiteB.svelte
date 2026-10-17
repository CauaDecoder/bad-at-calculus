<!--
  PROTOTYPE · Site B — "Ferramenta no herói" (warm paper).
  RCD: "your promise is the size of your proof" + value first — the hero IS the product:
  type a function and get the graph, the derivative and the steps before any signup.
-->
<script lang="ts">
  import { resolve } from '$app/paths';
  import { disciplines, functions } from '../mock';
  import MorphSelect from '../widgets/MorphSelect.svelte';
  import TangentGraph from '../widgets/TangentGraph.svelte';

  const options = [
    {
      key: 'cubic',
      input: 'x^3 - 3x + 1',
      steps: ['(x³)′ = 3x²', '(−3x)′ = −3', 'f′(x) = 3x² − 3'],
    },
    { key: 'square', input: 'x^2', steps: ['Regra da potência: (xⁿ)′ = n·xⁿ⁻¹', 'f′(x) = 2x'] },
    {
      key: 'sinSquare',
      input: 'sin(x^2)',
      steps: ['Regra da cadeia: (sen u)′ = cos u · u′', 'u = x², u′ = 2x', 'f′(x) = 2x·cos(x²)'],
    },
  ] as const;

  let selected = $state(0);
  const option = $derived(options[selected] as (typeof options)[number]);
  const fn = $derived(functions[option.key]);
  let course = $state('Cálculo A');
</script>

<div class="sb">
  <nav>
    <span class="brand">Vértice</span>
    <span class="spacer"></span>
    <a href="#github">GitHub</a>
    <a class="nav-cta" href={resolve('/prototype/inicio')}>Abrir o app →</a>
  </nav>

  <header class="hero">
    <h1>Digite uma função.</h1>
    <p class="sub">Isso já é a Vértice: gráfico, derivada e passos, na hora. Sem cadastro.</p>

    <div class="tool">
      <div class="input-row">
        <span class="fx">f(x) =</span>
        {#each options as o, i (o.key)}
          <button type="button" class:on={i === selected} onclick={() => (selected = i)}
            >{o.input}</button
          >
        {/each}
      </div>
      <div class="tool-body">
        <TangentGraph f={fn.f} df={fn.df} />
        <ol class="steps">
          {#each option.steps as step, i (`${option.key}-${i}`)}
            <li style:animation-delay={`${i * 140}ms`}>{step}</li>
          {/each}
        </ol>
      </div>
    </div>

    <p class="after">Agora imagine a sua lista inteira assim, <b>corrigida item por item</b>.</p>
    <div class="ctas">
      <a class="primary" href={resolve('/prototype/exercicio')}>Importar minha lista</a>
      <span class="trigger">PDF, planilha ou foto · fica só no seu computador</span>
    </div>
  </header>

  <section class="for">
    <h2>Feita para quem tem prova de exatas</h2>
    <p>Escolha seu curso:</p>
    <MorphSelect label="Curso" groups={disciplines} bind:value={course} />
    <div class="cols">
      <article>
        <b>Exatas</b>
        <p>Limites, derivadas, integrais, vetores 3D, matrizes e sistemas — com o gráfico vivo.</p>
      </article>
      <article>
        <b>Computação</b>
        <p>POO, algoritmos passo a passo e SQL rodando no navegador.</p>
      </article>
      <article>
        <b>Arquitetura</b>
        <p>Forças, equilíbrio, treliças e geometria descritiva em 3D.</p>
      </article>
    </div>
  </section>

  <footer>
    <span>Open source · MIT ou Apache 2.0 · feito por estudantes</span>
  </footer>
</div>

<style>
  .sb {
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
    --ms-bg: var(--surface);
    --ms-border: var(--border);
    --ms-hover: #f1ebdf;
    min-height: 100dvh;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: var(--text);
    background: var(--bg);
  }

  nav {
    display: flex;
    align-items: center;
    gap: 20px;
    max-width: 1100px;
    margin: 0 auto;
    padding: 20px 24px;
  }

  .brand {
    font-family: var(--serif);
    font-size: 22px;
    font-weight: 700;
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
    padding: 8px 14px;
    color: var(--text) !important;
    border: 1px solid var(--border);
    border-radius: 999px;
  }

  .hero {
    max-width: 1000px;
    margin: 0 auto;
    padding: 48px 24px 24px;
    text-align: center;
  }

  h1 {
    margin: 0;
    font-family: var(--serif);
    font-size: clamp(44px, 8vw, 88px);
    font-weight: 600;
    letter-spacing: -0.03em;
  }

  .sub {
    margin: 10px 0 32px;
    font-size: 19px;
    color: var(--muted);
  }

  .tool {
    padding: 16px;
    text-align: left;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 22px;
    box-shadow: 0 30px 80px rgb(80 60 30 / 0.12);
  }

  .input-row {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
    padding: 6px 6px 14px;
    border-bottom: 1px solid var(--border);
  }

  .fx {
    margin-right: 4px;
    font:
      600 18px 'JetBrains Mono Variable',
      monospace;
  }

  .input-row button {
    padding: 8px 14px;
    font:
      500 16px 'JetBrains Mono Variable',
      monospace;
    color: var(--muted);
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 10px;
    cursor: pointer;
    transition: all 200ms;
  }

  .input-row button.on {
    color: #fff;
    background: var(--text);
    border-color: var(--text);
  }

  .tool-body {
    display: grid;
    grid-template-columns: 1.5fr 1fr;
    gap: 24px;
    padding-top: 16px;
  }

  .steps {
    display: grid;
    gap: 10px;
    align-content: start;
    margin: 0;
    padding-left: 20px;
    font:
      500 15px 'JetBrains Mono Variable',
      monospace;
  }

  .steps li {
    animation: rise 360ms cubic-bezier(0.2, 0.9, 0.1, 1) both;
  }

  @keyframes rise {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
  }

  .after {
    margin: 36px 0 16px;
    font-size: 20px;
  }

  .ctas {
    display: grid;
    justify-items: center;
    gap: 10px;
  }

  .primary {
    padding: 16px 28px;
    font-weight: 600;
    color: #fff;
    text-decoration: none;
    background: var(--accent);
    border-radius: 999px;
  }

  .trigger {
    font-size: 13px;
    color: var(--muted);
  }

  .for {
    max-width: 1000px;
    margin: 0 auto;
    padding: 80px 24px 40px;
  }

  .for h2 {
    margin: 0;
    font-family: var(--serif);
    font-size: 34px;
  }

  .for > p {
    color: var(--muted);
  }

  .cols {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 14px;
    margin-top: 20px;
  }

  .cols article {
    padding: 20px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 16px;
  }

  .cols p {
    margin: 6px 0 0;
    color: var(--muted);
    font-size: 14px;
  }

  footer {
    padding: 40px 24px 120px;
    text-align: center;
    font-size: 13px;
    color: var(--muted);
  }

  @media (max-width: 760px) {
    .tool-body,
    .cols {
      grid-template-columns: 1fr;
    }
  }
</style>
