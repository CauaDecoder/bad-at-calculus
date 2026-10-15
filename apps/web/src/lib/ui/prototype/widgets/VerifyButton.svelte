<!--
  PROTOTYPE — "Verificar" button that morphs through states: idle → checking → correct / wrong
  (inspired by the metalforge pay interaction). The check mark draws itself.
-->
<script lang="ts" module>
  export type VerifyState = 'idle' | 'checking' | 'correct' | 'wrong';
</script>

<script lang="ts">
  interface Props {
    state: VerifyState;
    onclick: () => void;
  }

  let { state, onclick }: Props = $props();

  const labels: Record<VerifyState, string> = {
    idle: 'Verificar resposta',
    checking: 'Verificando…',
    correct: 'Correto',
    wrong: 'Ainda não',
  };
</script>

<button type="button" class={`verify ${state}`} {onclick} aria-live="polite">
  {#if state === 'checking'}
    <span class="spin" aria-hidden="true"></span>
  {:else if state === 'correct'}
    <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M5 12.5 10 17 19 7" /></svg>
  {:else if state === 'wrong'}
    <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M7 7l10 10M17 7 7 17" /></svg>
  {/if}
  <span>{labels[state]}</span>
</button>

<style>
  .verify {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    height: 44px;
    min-width: 188px;
    padding: 0 20px;
    font: 600 14px/1 inherit;
    font-family: inherit;
    color: var(--vb-on, #fff);
    background: var(--vb-bg, #4f46e5);
    border: 0;
    border-radius: 999px;
    cursor: pointer;
    transition:
      background 240ms cubic-bezier(0.2, 0, 0, 1),
      min-width 320ms cubic-bezier(0.2, 0.9, 0.1, 1),
      transform 120ms;
  }

  .verify:active {
    transform: scale(0.97);
  }

  .verify:focus-visible {
    outline: 3px solid var(--vb-ring, rgb(79 70 229 / 0.4));
    outline-offset: 2px;
  }

  .checking {
    min-width: 150px;
    opacity: 0.9;
  }

  .correct {
    min-width: 130px;
    background: var(--vb-ok, #16a34a);
  }

  .wrong {
    min-width: 130px;
    background: var(--vb-bad, #dc2626);
    animation: shake 320ms;
  }

  svg {
    width: 18px;
    height: 18px;
    fill: none;
    stroke: currentColor;
    stroke-width: 2.5;
    stroke-linecap: round;
    stroke-linejoin: round;
  }

  svg path {
    stroke-dasharray: 30;
    stroke-dashoffset: 30;
    animation: draw 360ms 80ms forwards cubic-bezier(0.2, 0, 0, 1);
  }

  .spin {
    width: 16px;
    height: 16px;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: spin 700ms linear infinite;
  }

  @keyframes draw {
    to {
      stroke-dashoffset: 0;
    }
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes shake {
    25% {
      transform: translateX(-4px);
    }
    75% {
      transform: translateX(4px);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .verify,
    svg path,
    .spin {
      animation: none;
      transition: none;
      stroke-dashoffset: 0;
    }
  }
</style>
