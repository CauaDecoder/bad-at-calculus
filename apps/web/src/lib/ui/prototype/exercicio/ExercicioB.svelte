<!--
  PROTOTYPE · Exercício B — "Cartão em foco" (warm paper).
  One part at a time, centered, keyboard first. On a correct answer the mastery line rises
  (peak-end rule: celebrate the activation moment instead of only confirming it).
-->
<script lang="ts">
  import { exercises } from '../mock';
  import { checkAnswer } from '../stub-check';
  import VerifyButton, { type VerifyState } from '../widgets/VerifyButton.svelte';
  import RollingNumber from '../widgets/RollingNumber.svelte';

  const queue = exercises.flatMap((ex) => ex.parts.map((p) => ({ ex, part: p })));
  let index = $state(0);
  const item = $derived(queue[index] as (typeof queue)[number]);
  let answer = $state('');
  let verdict: VerifyState = $state('idle');
  let message = $state('');
  let hint = $state(false);
  let mastery = $state(41);
  let celebrated = $state(false);

  function verify(): void {
    const result = checkAnswer(`${item.ex.number}${item.part.label}`, answer);
    if (result.kind === 'invalid') {
      message = result.message;
      return;
    }
    verdict = 'checking';
    setTimeout(() => {
      if (result.kind === 'correct') {
        verdict = 'correct';
        message = '';
        mastery = Math.min(100, mastery + 6);
        celebrated = true;
      } else {
        verdict = 'wrong';
        message = result.message;
      }
    }, 600);
  }

  function next(): void {
    index = (index + 1) % queue.length;
    answer = '';
    verdict = 'idle';
    message = '';
    hint = false;
    celebrated = false;
  }

  function onKey(event: KeyboardEvent): void {
    if (event.key === 'Enter') {
      event.preventDefault();
      if (verdict === 'correct') next();
      else verify();
    }
    if (event.key === '?') hint = true;
  }
</script>

<div class="eb">
  <header>
    <a href="/prototype/inicio?variant=B" class="close" aria-label="Sair">✕</a>
    <div class="progress"><span style:width={`${((index + 1) / queue.length) * 100}%`}></span></div>
    <span class="count">{index + 1}/{queue.length}</span>
  </header>

  <main>
    {#key index}
      <article class="card">
        <p class="eyebrow">Exercício {item.ex.number}{item.part.label} · Derivadas</p>
        <p class="statement">{item.ex.statement}</p>
        <h1>{item.part.prompt}</h1>

        <input
          bind:value={answer}
          onkeydown={onKey}
          placeholder="Digite e aperte Enter"
          aria-label="Sua resposta"
          autocomplete="off"
        />

        {#if message}<p class="msg">{message}</p>{/if}
        {#if hint}<p class="hint">💡 {item.part.hint}</p>{/if}

        {#if celebrated}
          <div class="celebrate">
            <svg viewBox="0 0 200 60" aria-hidden="true">
              <path d="M0 52 C 40 50, 70 44, 100 36 S 160 10, 200 6" />
            </svg>
            <div>
              <span class="pct"><RollingNumber value={`${mastery}%`} /></span>
              <span>de domínio em Derivadas (+6)</span>
            </div>
          </div>
        {/if}

        <div class="row">
          {#if verdict === 'correct'}
            <button type="button" class="next" onclick={next}>Próximo item →</button>
          {:else}
            <VerifyButton state={verdict} onclick={verify} />
          {/if}
          <button type="button" class="link" onclick={() => (hint = true)}
            >Preciso de uma dica</button
          >
        </div>
      </article>
    {/key}

    <p class="keys">
      <kbd>Enter</kbd> verificar / próximo · <kbd>?</kbd> dica · erro de sintaxe não conta
    </p>
  </main>
</div>

<style>
  .eb {
    --bg: #f6f4ef;
    --surface: #fffdf9;
    --border: #e8e2d6;
    --text: #1f1b16;
    --muted: #7a7268;
    --accent: #d9541e;
    --serif: 'Iowan Old Style', 'Palatino Linotype', Palatino, Georgia, serif;
    --vb-bg: var(--text);
    --vb-ok: #2f7d4f;
    --vb-bad: #b4472a;
    min-height: 100dvh;
    font-family: 'Inter Variable', system-ui, sans-serif;
    color: var(--text);
    background: var(--bg);
  }

  header {
    display: flex;
    align-items: center;
    gap: 18px;
    max-width: 760px;
    margin: 0 auto;
    padding: 24px 24px 0;
  }

  .close {
    color: var(--muted);
    text-decoration: none;
    font-size: 18px;
  }

  .progress {
    flex: 1;
    height: 8px;
    overflow: hidden;
    background: #e9e2d5;
    border-radius: 999px;
  }

  .progress span {
    display: block;
    height: 100%;
    background: var(--accent);
    border-radius: inherit;
    transition: width 420ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  .count {
    font:
      500 13px 'JetBrains Mono Variable',
      monospace;
    color: var(--muted);
  }

  main {
    display: grid;
    place-items: center;
    gap: 16px;
    min-height: calc(100dvh - 60px);
    padding: 24px 24px 120px;
  }

  .card {
    width: min(640px, 100%);
    padding: 40px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 24px;
    box-shadow: 0 20px 60px rgb(80 60 30 / 0.08);
    animation: enter 380ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  @keyframes enter {
    from {
      opacity: 0;
      transform: translateY(14px) scale(0.98);
    }
  }

  .eyebrow {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--accent);
  }

  .statement {
    margin: 14px 0 4px;
    color: var(--muted);
  }

  h1 {
    margin: 0 0 24px;
    font-family: var(--serif);
    font-size: 30px;
    font-weight: 600;
    line-height: 1.2;
  }

  input {
    width: 100%;
    height: 60px;
    padding: 0 18px;
    font:
      500 20px 'JetBrains Mono Variable',
      monospace;
    color: var(--text);
    background: #fbf8f2;
    border: 1.5px solid var(--border);
    border-radius: 14px;
    outline: none;
  }

  input:focus {
    border-color: var(--text);
  }

  .msg,
  .hint {
    margin: 14px 0 0;
    padding: 12px 14px;
    font-size: 14px;
    border-radius: 12px;
  }

  .msg {
    color: #8a3a20;
    background: #fbe9e0;
  }

  .hint {
    background: #f6edcf;
  }

  .celebrate {
    display: flex;
    gap: 16px;
    align-items: center;
    margin-top: 18px;
    padding: 14px;
    background: #eaf3ec;
    border-radius: 14px;
    animation: enter 420ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  .celebrate svg {
    width: 160px;
    height: 48px;
    fill: none;
    stroke: #2f7d4f;
    stroke-width: 3;
    stroke-linecap: round;
  }

  .celebrate path {
    stroke-dasharray: 240;
    stroke-dashoffset: 240;
    animation: draw 900ms 120ms forwards cubic-bezier(0.2, 0, 0, 1);
  }

  @keyframes draw {
    to {
      stroke-dashoffset: 0;
    }
  }

  .celebrate div {
    display: grid;
    font-size: 13px;
    color: #2f5d40;
  }

  .pct {
    font-size: 28px;
    font-weight: 600;
  }

  .row {
    display: flex;
    gap: 16px;
    align-items: center;
    margin-top: 22px;
  }

  .next {
    height: 44px;
    padding: 0 22px;
    font: inherit;
    font-weight: 600;
    color: #fff;
    background: #2f7d4f;
    border: 0;
    border-radius: 999px;
    cursor: pointer;
  }

  .link {
    font: inherit;
    font-size: 14px;
    color: var(--muted);
    background: none;
    border: 0;
    text-decoration: underline;
    cursor: pointer;
  }

  .keys {
    font-size: 12px;
    color: var(--muted);
  }

  kbd {
    padding: 2px 6px;
    font:
      11px 'JetBrains Mono Variable',
      monospace;
    background: #ece5d8;
    border-radius: 5px;
  }

  @media (max-width: 640px) {
    .card {
      padding: 24px;
    }

    h1 {
      font-size: 24px;
    }
  }
</style>
