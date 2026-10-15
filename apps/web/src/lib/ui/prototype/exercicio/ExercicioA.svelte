<!--
  PROTOTYPE · Exercício A — "Laboratório dividido" (light neutral).
  Statement and answer on the left, the live graph of the same function on the right.
  RCD: one dominant primary action, click-trigger microcopy, hints in levels, common-mistake
  feedback, steps only after an attempt.
-->
<script lang="ts">
  import { exercises, functions } from '../mock';
  import { checkAnswer } from '../stub-check';
  import TangentGraph from '../widgets/TangentGraph.svelte';
  import VerifyButton, { type VerifyState } from '../widgets/VerifyButton.svelte';

  const exercise = exercises[0] as (typeof exercises)[number];
  let partIndex = $state(0);
  const part = $derived(exercise.parts[partIndex] as (typeof exercise.parts)[number]);
  let answer = $state('');
  let verdict: VerifyState = $state('idle');
  let message = $state('');
  let hintsShown = $state(0);
  let attempts = $state(0);
  let x0 = $state(1.5);

  function verify(): void {
    const result = checkAnswer(`${exercise.number}${part.label}`, answer);
    if (result.kind === 'invalid') {
      message = result.message;
      return;
    }
    verdict = 'checking';
    attempts += 1;
    setTimeout(() => {
      verdict = result.kind === 'correct' ? 'correct' : 'wrong';
      message = result.kind === 'correct' ? 'Isso! Veja no gráfico onde f′ zera.' : result.message;
      if (result.kind === 'correct') x0 = 1;
    }, 650);
  }

  function selectPart(i: number): void {
    partIndex = i;
    answer = '';
    verdict = 'idle';
    message = '';
    hintsShown = 0;
  }
</script>

<div class="ea">
  <header>
    <a href="/prototype/inicio?variant=A" class="back">← Derivadas — exemplo</a>
    <div class="dots" aria-label="Item 7 de 19">
      {#each Array.from({ length: 19 }, (_v, n) => n) as i (i)}
        <span class:done={i < 6} class:now={i === 6}></span>
      {/each}
    </div>
    <span class="meta">Item 7 de 19 · P1 em 9 dias</span>
  </header>

  <div class="split">
    <section class="left">
      <p class="num">Exercício {exercise.number}</p>
      <p class="statement">{exercise.statement}</p>

      <div class="parts" role="tablist">
        {#each exercise.parts as p, i (p.label)}
          <button
            type="button"
            role="tab"
            aria-selected={i === partIndex}
            onclick={() => selectPart(i)}
          >
            {p.label})
          </button>
        {/each}
      </div>

      <p class="prompt">{part.prompt}</p>

      <label class="field">
        <span>Sua resposta</span>
        <input
          bind:value={answer}
          placeholder={part.label === 'a' ? 'ex.: {-1, 1}' : 'ex.: y = 2x + 3'}
          onkeydown={(e) => e.key === 'Enter' && verify()}
          oninput={() => {
            if (verdict !== 'checking') verdict = 'idle';
          }}
        />
      </label>

      <div class="actions">
        <VerifyButton state={verdict} onclick={verify} />
        <button
          type="button"
          class="hint"
          onclick={() => (hintsShown = Math.min(3, hintsShown + 1))}
        >
          Dica {hintsShown > 0 ? `(${hintsShown}/3)` : ''}
        </button>
      </div>
      <p class="trigger">
        Enter verifica · erro de sintaxe não conta tentativa · {attempts} tentativa(s)
      </p>

      {#if message}<p class={`msg ${verdict}`}>{message}</p>{/if}

      {#if hintsShown > 0}
        <div class="hints">
          <p>💡 {part.hint}</p>
          {#if hintsShown > 1}<p>Derive: f′(x) = 3x² − 3.</p>{/if}
          {#if hintsShown > 2}
            <p>
              {part.label === 'a'
                ? 'Resolva 3x² − 3 = 0.'
                : 'f′(2) = 9 e f(2) = 3: use y − 3 = 9(x − 2).'}
            </p>
          {/if}
        </div>
      {/if}

      {#if verdict === 'correct' || attempts >= 2}
        <details class="steps" open={verdict === 'correct'}>
          <summary>Solução passo a passo (gerada pelo motor)</summary>
          <ol>
            <li>Regra da potência: (x³)′ = 3x², (−3x)′ = −3, (1)′ = 0.</li>
            <li>f′(x) = 3x² − 3.</li>
            <li>f′(x) = 0 ⇔ x² = 1 ⇔ x = ±1.</li>
          </ol>
        </details>
      {/if}
    </section>

    <section class="right">
      <div class="graph-head">
        <b>f(x) = {functions.cubic.label}</b>
        <span>f′ tracejada · arraste o ponto</span>
      </div>
      <div class="graph">
        <TangentGraph f={functions.cubic.f} df={functions.cubic.df} bind:x0 autoplay={false} />
      </div>
      <p class="note">
        O motor desenha o mesmo f do enunciado. Onde a reta verde fica horizontal, f′(x₀) = 0.
      </p>
    </section>
  </div>
</div>

<style>
  .ea {
    --bg: #f7f7f8;
    --surface: #fff;
    --border: #e7e7ea;
    --text: #18181b;
    --muted: #6b6b76;
    --accent: #4f46e5;
    min-height: 100dvh;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: var(--text);
    background: var(--bg);
  }

  header {
    display: flex;
    align-items: center;
    gap: 24px;
    padding: 14px 24px;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
  }

  .back {
    font-weight: 500;
    color: var(--text);
    text-decoration: none;
  }

  .dots {
    display: flex;
    gap: 4px;
    flex: 1;
    justify-content: center;
  }

  .dots span {
    width: 18px;
    height: 6px;
    background: #e4e4e7;
    border-radius: 999px;
  }

  .dots .done {
    background: #16a34a;
  }

  .dots .now {
    background: var(--accent);
    width: 28px;
  }

  .meta {
    font-size: 13px;
    color: var(--muted);
  }

  .split {
    display: grid;
    grid-template-columns: minmax(360px, 1fr) 1.2fr;
    min-height: calc(100dvh - 57px);
  }

  .left {
    padding: 32px 40px 110px;
    background: var(--surface);
    border-right: 1px solid var(--border);
  }

  .num {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--accent);
  }

  .statement {
    margin: 8px 0 20px;
    font-size: 20px;
    font-weight: 500;
  }

  .parts {
    display: flex;
    gap: 6px;
    margin-bottom: 12px;
  }

  .parts button {
    width: 40px;
    height: 32px;
    font: inherit;
    font-weight: 600;
    color: var(--muted);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
  }

  .parts button[aria-selected='true'] {
    color: var(--accent);
    background: #eef2ff;
    border-color: #c7d2fe;
  }

  .prompt {
    margin: 0 0 18px;
    font-size: 16px;
  }

  .field {
    display: grid;
    gap: 6px;
    font-size: 13px;
    color: var(--muted);
  }

  .field input {
    height: 48px;
    padding: 0 14px;
    font:
      500 17px 'JetBrains Mono Variable',
      monospace;
    color: var(--text);
    background: var(--surface);
    border: 1.5px solid var(--border);
    border-radius: 10px;
    outline: none;
    transition: border-color 160ms;
  }

  .field input:focus {
    border-color: var(--accent);
  }

  .actions {
    display: flex;
    gap: 10px;
    align-items: center;
    margin-top: 16px;
  }

  .hint {
    height: 44px;
    padding: 0 16px;
    font: inherit;
    font-weight: 500;
    color: var(--text);
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 999px;
    cursor: pointer;
  }

  .trigger {
    margin: 8px 0 0;
    font-size: 12px;
    color: var(--muted);
  }

  .msg {
    margin: 16px 0 0;
    padding: 12px 14px;
    font-size: 14px;
    border-radius: 10px;
    background: #f4f4f5;
    animation: rise 260ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  .msg.correct {
    color: #166534;
    background: #f0fdf4;
  }

  .msg.wrong {
    color: #9a3412;
    background: #fff7ed;
  }

  .hints {
    margin-top: 14px;
    padding: 4px 14px;
    font-size: 14px;
    background: #fefce8;
    border: 1px solid #fef08a;
    border-radius: 10px;
  }

  .steps {
    margin-top: 16px;
    font-size: 14px;
  }

  .steps summary {
    font-weight: 600;
    cursor: pointer;
  }

  .right {
    padding: 32px 40px 110px;
  }

  .graph-head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 12px;
    font-size: 14px;
  }

  .graph-head span,
  .note {
    font-size: 13px;
    color: var(--muted);
  }

  .graph {
    padding: 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 14px;
  }

  @keyframes rise {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
  }

  @media (max-width: 900px) {
    .split {
      grid-template-columns: 1fr;
    }

    .left,
    .right {
      padding: 20px 16px;
    }

    .right {
      padding-bottom: 110px;
    }

    .dots,
    .meta {
      display: none;
    }
  }
</style>
