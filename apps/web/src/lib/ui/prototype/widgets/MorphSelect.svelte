<!--
  PROTOTYPE — select whose pill trigger grows into the list, with a gliding highlight and
  type-ahead (inspired by uiarc morph-select).
-->
<script lang="ts">
  import { cubicOut } from 'svelte/easing';

  interface Props {
    label: string;
    groups: readonly { group: string; items: readonly string[] }[];
    value?: string;
  }

  let { label, groups, value = $bindable(groups[0]?.items[0] ?? '') }: Props = $props();

  let open = $state(false);
  let highlighted = $state(0);
  let trigger: HTMLButtonElement | undefined = $state();
  const flat = $derived(groups.flatMap((g) => g.items));

  function grow(node: HTMLElement) {
    const from = trigger?.getBoundingClientRect();
    const to = node.getBoundingClientRect();
    const w0 = from?.width ?? to.width;
    const h0 = from?.height ?? 36;
    return {
      duration: 260,
      easing: cubicOut,
      css: (t: number) =>
        `width:${w0 + (to.width - w0) * t}px;height:${h0 + (to.height - h0) * t}px;` +
        `border-radius:${18 - 6 * t}px;overflow:hidden`,
    };
  }

  function choose(item: string): void {
    value = item;
    open = false;
    trigger?.focus();
  }

  function onKey(event: KeyboardEvent): void {
    if (event.key === 'Escape') open = false;
    if (event.key === 'ArrowDown') highlighted = (highlighted + 1) % flat.length;
    if (event.key === 'ArrowUp') highlighted = (highlighted - 1 + flat.length) % flat.length;
    if (event.key === 'Enter') choose(flat[highlighted] ?? value);
    if (event.key.length === 1) {
      const i = flat.findIndex((s) => s.toLowerCase().startsWith(event.key.toLowerCase()));
      if (i >= 0) highlighted = i;
    }
    event.stopPropagation();
  }
</script>

<div class="morph">
  <button
    bind:this={trigger}
    type="button"
    class="trigger"
    aria-haspopup="listbox"
    aria-expanded={open}
    onclick={() => {
      highlighted = Math.max(0, flat.indexOf(value));
      open = true;
    }}
  >
    <span class="lbl">{label}</span>
    {value}
    <span aria-hidden="true">▾</span>
  </button>

  {#if open}
    <div
      class="list"
      role="listbox"
      tabindex="-1"
      aria-label={label}
      transition:grow
      onkeydown={onKey}
    >
      {#each groups as g (g.group)}
        <p class="group">{g.group}</p>
        {#each g.items as item (item)}
          {@const i = flat.indexOf(item)}
          <button
            type="button"
            role="option"
            aria-selected={item === value}
            class:hl={i === highlighted}
            onpointerenter={() => (highlighted = i)}
            onclick={() => choose(item)}
          >
            {item}
            {#if item === value}<span aria-hidden="true">✓</span>{/if}
          </button>
        {/each}
      {/each}
    </div>
    <button class="scrim" type="button" aria-label="Fechar" onclick={() => (open = false)}></button>
  {/if}
</div>

<style>
  .morph {
    position: relative;
    display: inline-block;
  }

  .trigger {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    height: 36px;
    padding: 0 14px;
    font: 500 13px/1 inherit;
    font-family: inherit;
    color: var(--ms-text, #18181b);
    background: var(--ms-bg, #fff);
    border: 1px solid var(--ms-border, #e4e4e7);
    border-radius: 18px;
    cursor: pointer;
  }

  .lbl {
    color: var(--ms-muted, #71717a);
  }

  .list {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 20;
    width: 240px;
    padding: 6px;
    background: var(--ms-bg, #fff);
    border: 1px solid var(--ms-border, #e4e4e7);
    border-radius: 12px;
    box-shadow: 0 16px 40px rgb(0 0 0 / 0.16);
  }

  .group {
    margin: 8px 8px 4px;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    color: var(--ms-muted, #71717a);
  }

  [role='option'] {
    display: flex;
    justify-content: space-between;
    width: 100%;
    padding: 8px 10px;
    font: inherit;
    font-size: 13px;
    text-align: left;
    color: var(--ms-text, #18181b);
    background: transparent;
    border: 0;
    border-radius: 8px;
    cursor: pointer;
    transition: background 120ms;
  }

  [role='option'].hl {
    background: var(--ms-hover, #f4f4f5);
  }

  .scrim {
    position: fixed;
    inset: 0;
    z-index: 10;
    background: transparent;
    border: 0;
  }
</style>
