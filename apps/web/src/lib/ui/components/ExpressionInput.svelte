<!--
  Monospace input for f(x) with inline syntax feedback (no modals, no "Calcular" button).
  See docs/04-design/principios-ux.md.
-->
<script lang="ts">
  import type { EngineError } from '$lib/domain/engine-error';
  import { describeEngineError } from '$lib/ui/copy/engine-messages';
  import ErrorNote from './ErrorNote.svelte';

  interface Props {
    value: string;
    error: EngineError | null;
    prefix?: string;
    oninput: (value: string) => void;
  }

  let { value, error, prefix = 'f(x) =', oninput }: Props = $props();

  const inputId = `expression-${Math.random().toString(36).slice(2, 8)}`;
  const message = $derived(error ? describeEngineError(error, value) : null);
</script>

<div class="field" class:invalid={message !== null}>
  <label class="prefix" for={inputId}>{prefix}</label>
  <input
    id={inputId}
    class="input"
    {value}
    oninput={(event) => oninput(event.currentTarget.value)}
    spellcheck="false"
    autocomplete="off"
    autocapitalize="off"
    aria-invalid={message !== null}
  />
</div>

{#if message}
  <ErrorNote {message} />
{/if}

<style>
  .field {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: 0 var(--space-3);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    transition: border-color var(--duration-fast) var(--easing-standard);
  }

  .field:focus-within {
    border-color: var(--color-accent);
    box-shadow: 0 0 0 3px var(--color-accent-soft);
  }

  .field.invalid {
    border-color: var(--color-danger);
  }

  .prefix {
    flex: none;
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-text-muted);
  }

  .input {
    flex: 1;
    min-width: 0;
    padding: var(--space-2) 0;
    font-family: var(--font-mono);
    font-size: var(--text-md);
    color: var(--color-text);
    background: transparent;
    border: 0;
    outline: none;
  }
</style>
