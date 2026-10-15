<script lang="ts">
  interface Props {
    checked: boolean;
    label: string;
    disabled?: boolean;
    onchange: (checked: boolean) => void;
  }

  let { checked, label, disabled = false, onchange }: Props = $props();
</script>

<button
  type="button"
  role="switch"
  class="switch"
  aria-checked={checked}
  aria-label={label}
  {disabled}
  onclick={() => onchange(!checked)}
>
  <span class="thumb"></span>
</button>

<style>
  .switch {
    position: relative;
    flex: none;
    width: 30px;
    height: 18px;
    padding: 0;
    background: var(--color-border);
    border: 0;
    border-radius: var(--radius-full);
    cursor: pointer;
    transition: background var(--duration-normal) var(--easing-standard);
  }

  .switch[aria-checked='true'] {
    background: var(--color-accent);
  }

  .switch:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .switch:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: 2px;
  }

  .thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 14px;
    height: 14px;
    background: #fff;
    border-radius: 50%;
    box-shadow: var(--shadow-sm);
    transition: transform var(--duration-normal) var(--easing-standard);
  }

  .switch[aria-checked='true'] .thumb {
    transform: translateX(12px);
  }
</style>
