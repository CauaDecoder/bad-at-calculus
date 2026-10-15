<!--
  Compact text field for a number typed as math: `2`, `-1/2`, `sqrt(3)`, `pi/4`.
  The engine evaluates it, so fractions and roots stay exact.
-->
<script lang="ts">
  interface Props {
    value: string;
    label: string;
    invalid?: boolean;
    width?: string;
    placeholder?: string;
    oninput: (value: string) => void;
  }

  let {
    value,
    label,
    invalid = false,
    width = '100%',
    placeholder = '0',
    oninput,
  }: Props = $props();
</script>

<input
  class="number-field"
  class:invalid
  style:width
  {value}
  {placeholder}
  aria-label={label}
  aria-invalid={invalid}
  spellcheck="false"
  autocomplete="off"
  inputmode="text"
  oninput={(event) => oninput(event.currentTarget.value)}
/>

<style>
  .number-field {
    min-width: 0;
    height: 34px;
    padding: 0 var(--space-2);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-variant-numeric: tabular-nums;
    text-align: center;
    color: var(--color-text);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    transition: border-color var(--duration-fast) var(--easing-standard);
  }

  .number-field:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 3px var(--color-accent-soft);
  }

  .number-field.invalid {
    border-color: var(--color-danger);
  }

  .number-field::placeholder {
    color: var(--color-text-muted);
    opacity: 0.5;
  }
</style>
