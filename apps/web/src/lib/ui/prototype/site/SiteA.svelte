<!--
  PROTOTYPE · Site A — "Problema → prova → CTA" (light, classic conversion structure).
  RCD: 5-second test (what / for whom / why now), pain-led headline, real product as proof,
  CTA microcopy + click trigger, FAQ that kills the top objections, CTA repeated.
-->
<script lang="ts">
  import { resolve } from '$app/paths';
  import { functions } from '../mock';
  import TangentGraph from '../widgets/TangentGraph.svelte';

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
</script>

<div class="sa">
  <nav>
    <span class="brand"><span class="mark"></span>Vértice</span>
    <a href="#como">Como funciona</a>
    <a href="#materias">Matérias</a>
    <a href="#faq">Perguntas</a>
    <a class="gh" href="#github">★ GitHub</a>
    <a class="nav-cta" href={resolve('/prototype/inicio')}>Abrir o app</a>
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
        <a class="primary" href={resolve('/prototype/exercicio')}>Resolver um exercício agora</a>
        <a class="secondary" href={resolve('/prototype/inicio')}>Abrir o laboratório</a>
      </div>
      <p class="trigger">Grátis · sem cadastro · funciona offline</p>
    </div>
    <div class="demo">
      <div class="demo-bar"><span></span><span></span><span></span> Derivadas · item 7a</div>
      <div class="demo-body">
        <p>Em quais x a tangente a f(x) = x³ − 3x + 1 é horizontal?</p>
        <div class="fake-input">{'{-1, 1}'} <span class="ok">✓ Correto</span></div>
        <TangentGraph f={functions.cubic.f} df={functions.cubic.df} readout={false} />
      </div>
    </div>
  </header>

  <section class="proof">
    <span><b>7</b> tópicos prontos</span>
    <span><b>0</b> cadastros</span>
    <span>motor em <b>Rust</b> no navegador</span>
    <span>licença <b>MIT</b></span>
  </section>

  <section class="pains" id="como">
    <h2>Estudar exatas não deveria ser adivinhar onde errou</h2>
    <div class="three">
      {#each pains as p (p.title)}
        <article>
          <h3>{p.title}</h3>
          <p>{p.body}</p>
        </article>
      {/each}
    </div>
  </section>

  <section class="features">
    {#each features as f, i (f.title)}
      <article class:flip={i % 2 === 1}>
        <div>
          <span class="tag">{f.tag}</span>
          <h3>{f.title}</h3>
          <p>{f.body}</p>
        </div>
        <div class="shot">
          {#if i === 0}
            <TangentGraph f={functions.square.f} df={functions.square.df} readout={false} />
          {:else if i === 1}
            <div class="fake-input big">5x − y = 16 <span class="ok">✓ mesma reta</span></div>
            <div class="fake-input big wrong">cos(x²) <span>faltou × 2x</span></div>
          {:else}
            <div class="cards">
              <span>Regra da cadeia · revisar hoje</span>
              <span>Limites laterais · em 3 dias</span>
              <span>Reta tangente · em 6 dias</span>
            </div>
          {/if}
        </div>
      </article>
    {/each}
  </section>

  <section class="faq" id="faq">
    <h2>Perguntas</h2>
    {#each faq as item (item.q)}
      <details>
        <summary>{item.q}</summary>
        <p>{item.a}</p>
      </details>
    {/each}
  </section>

  <section class="final">
    <h2>Sua próxima lista, corrigida na hora.</h2>
    <a class="primary" href={resolve('/prototype/exercicio')}>Resolver um exercício agora</a>
    <p class="trigger">Grátis · sem cadastro · funciona offline</p>
  </section>
</div>

<style>
  .sa {
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

  .demo-body {
    display: grid;
    gap: 12px;
    padding: 20px;
  }

  .demo-body p {
    margin: 0;
    font-weight: 500;
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
    margin-bottom: 10px;
    font-size: 17px;
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
  .features,
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
    padding: 20px;
    background: var(--soft);
    border: 1px solid var(--border);
    border-radius: 16px;
  }

  .cards {
    display: grid;
    gap: 10px;
  }

  .cards span {
    padding: 14px;
    background: #fff;
    border: 1px solid var(--border);
    border-radius: 10px;
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
  }
</style>
