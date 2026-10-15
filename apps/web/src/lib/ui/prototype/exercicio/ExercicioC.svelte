<!--
  PROTOTYPE · Exercício C — "Caderno" (dark expressive).
  The whole list scrolls like a notebook; every part answers inline; a sticky rail shows the
  status of each item (Zeigarnik effect: open loops stay visible).
-->
<script lang="ts">
  import { exercises } from '../mock';
  import { checkAnswer } from '../stub-check';

  type Status = 'open' | 'correct' | 'wrong';
  const answers: Record<string, string> = $state({});
  const status: Record<string, Status> = $state({});
  const messages: Record<string, string> = $state({});

  const keys = exercises.flatMap((ex) => ex.parts.map((p) => `${ex.number}${p.label}`));
  const solved = $derived(keys.filter((k) => status[k] === 'correct').length);

  function verify(key: string): void {
    const result = checkAnswer(key, answers[key] ?? '');
    if (result.kind === 'invalid') {
      messages[key] = result.message;
      return;
    }
    status[key] = result.kind === 'correct' ? 'correct' : 'wrong';
    messages[key] = result.kind === 'correct' ? 'Correto.' : result.message;
  }
</script>

<div class="ec">
  <aside class="rail">
    <a href="/prototype/inicio?variant=C" class="brand"><span class="mark"></span>vértice</a>
    <p class="list">Derivadas — exemplo</p>
    <div class="ring" style:--p={`${(solved / keys.length) * 100}%`}>
      <span>{solved}/{keys.length}</span>
    </div>
    <ol>
      {#each keys as key (key)}
        <li class={status[key] ?? 'open'}>
          <a href={`#item-${key}`}>{key}</a>
        </li>
      {/each}
    </ol>
    <p class="exam">P1 em <b>9 dias</b></p>
  </aside>

  <main>
    <header>
      <h1>Derivadas — exemplo</h1>
      <p>Responda em qualquer ordem. Itens abertos ficam marcados na lateral.</p>
    </header>

    {#each exercises as ex (ex.number)}
      <section class="item">
        <span class="n">{ex.number}</span>
        <div class="body">
          <p class="statement">{ex.statement}</p>
          {#each ex.parts as part (part.label)}
            {@const key = `${ex.number}${part.label}`}
            <div class={`part ${status[key] ?? 'open'}`} id={`item-${key}`}>
              <p><b>{part.label})</b> {part.prompt}</p>
              <div class="answer">
                <input
                  bind:value={answers[key]}
                  placeholder="resposta"
                  aria-label={`Resposta do item ${key}`}
                  onkeydown={(e) => e.key === 'Enter' && verify(key)}
                />
                <button type="button" onclick={() => verify(key)}>
                  {status[key] === 'correct' ? '✓' : 'Verificar'}
                </button>
              </div>
              {#if messages[key]}<p class="msg">{messages[key]}</p>{/if}
            </div>
          {/each}
        </div>
      </section>
    {/each}
  </main>
</div>

<style>
  .ec {
    --bg: #0e0f11;
    --surface: #16181c;
    --border: #25282e;
    --text: #f2f2f3;
    --muted: #8d8f98;
    --accent: #ff671c;
    --ok: #3ddc97;
    --bad: #ff8a65;
    display: grid;
    grid-template-columns: 240px 1fr;
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
    gap: 16px;
    height: 100dvh;
    padding: 22px 18px;
    border-right: 1px solid var(--border);
  }

  .brand {
    display: flex;
    gap: 8px;
    align-items: center;
    font-weight: 600;
    color: var(--text);
    text-decoration: none;
  }

  .mark {
    width: 16px;
    height: 16px;
    background: var(--accent);
    clip-path: polygon(50% 0, 100% 100%, 0 100%);
  }

  .list {
    margin: 0;
    font-size: 13px;
    color: var(--muted);
  }

  .ring {
    display: grid;
    place-items: center;
    width: 96px;
    height: 96px;
    border-radius: 50%;
    background: conic-gradient(var(--ok) var(--p), #22252b 0);
    transition: --p 400ms;
  }

  .ring span {
    display: grid;
    place-items: center;
    width: 76px;
    height: 76px;
    font:
      600 18px 'JetBrains Mono Variable',
      monospace;
    background: var(--bg);
    border-radius: 50%;
  }

  ol {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 6px;
    margin: 0;
    padding: 0;
    list-style: none;
  }

  ol a {
    display: grid;
    place-items: center;
    height: 34px;
    font:
      500 12px 'JetBrains Mono Variable',
      monospace;
    color: var(--muted);
    text-decoration: none;
    border: 1px dashed #3a3e46;
    border-radius: 8px;
  }

  li.correct a {
    color: #0e0f11;
    background: var(--ok);
    border: 0;
  }

  li.wrong a {
    color: var(--bad);
    border: 1px solid var(--bad);
  }

  .exam {
    margin-top: auto;
    font-size: 13px;
    color: var(--muted);
  }

  .exam b {
    color: var(--accent);
  }

  main {
    max-width: 820px;
    padding: 40px 48px 140px;
  }

  header h1 {
    margin: 0;
    font-size: 40px;
    letter-spacing: -0.03em;
  }

  header p {
    margin: 8px 0 32px;
    color: var(--muted);
  }

  .item {
    display: grid;
    grid-template-columns: 56px 1fr;
    gap: 8px;
    padding: 28px 0;
    border-top: 1px solid var(--border);
  }

  .n {
    font:
      600 28px 'JetBrains Mono Variable',
      monospace;
    color: var(--accent);
  }

  .statement {
    margin: 4px 0 14px;
    font-size: 18px;
  }

  .part {
    margin-bottom: 14px;
    padding: 14px 16px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-left: 3px solid #3a3e46;
    border-radius: 12px;
    transition: border-color 240ms;
  }

  .part.correct {
    border-left-color: var(--ok);
  }

  .part.wrong {
    border-left-color: var(--bad);
  }

  .part p {
    margin: 0 0 10px;
  }

  .answer {
    display: flex;
    gap: 8px;
  }

  .answer input {
    flex: 1;
    height: 42px;
    padding: 0 12px;
    font:
      500 15px 'JetBrains Mono Variable',
      monospace;
    color: var(--text);
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 10px;
    outline: none;
  }

  .answer input:focus {
    border-color: var(--accent);
  }

  .answer button {
    min-width: 96px;
    padding: 0 14px;
    font: inherit;
    font-weight: 600;
    color: #0e0f11;
    background: var(--accent);
    border: 0;
    border-radius: 10px;
    cursor: pointer;
  }

  .part.correct .answer button {
    background: var(--ok);
  }

  .msg {
    margin: 10px 0 0 !important;
    font-size: 13px;
    color: var(--muted);
  }

  .part.wrong .msg {
    color: var(--bad);
  }

  @media (max-width: 800px) {
    .ec {
      grid-template-columns: 1fr;
    }

    .rail {
      display: none;
    }

    main {
      padding: 24px 16px 120px;
    }
  }
</style>
