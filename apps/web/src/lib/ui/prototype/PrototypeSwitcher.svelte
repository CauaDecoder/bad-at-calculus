<!--
  PROTOTYPE — floating bar to flip between design variants (?variant=A|B|C) and screens.
  Only rendered in development builds.
-->
<script lang="ts">
  import { dev } from '$app/environment';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';

  interface Props {
    variants: readonly { key: string; name: string }[];
    /** Variant shown when the URL has no ?variant= (the chosen one). */
    defaultKey?: string;
  }

  let { variants, defaultKey = 'A' }: Props = $props();

  const screens = [
    { href: '/prototype/inicio', label: 'Início', chosen: 'A' },
    { href: '/prototype/exercicio', label: 'Exercício', chosen: 'D' },
    { href: '/prototype/site', label: 'Site', chosen: 'D' },
  ];

  const currentKey = $derived(page.url.searchParams.get('variant') ?? defaultKey);
  const index = $derived(
    Math.max(
      0,
      variants.findIndex((v) => v.key === currentKey),
    ),
  );
  const current = $derived(variants[index]);

  function select(offset: number): void {
    const next = variants[(index + offset + variants.length) % variants.length];
    if (!next) return;
    const url = new URL(page.url);
    url.searchParams.set('variant', next.key);
    goto(url, { replaceState: true, noScroll: true, keepFocus: true });
  }

  function onKey(event: KeyboardEvent): void {
    const target = event.target;
    const typing = 'input, textarea, [contenteditable="true"], [role="listbox"]';
    if (target instanceof Element && target.closest(typing)) return;
    if (event.key === 'ArrowLeft') select(-1);
    if (event.key === 'ArrowRight') select(1);
  }
</script>

<svelte:window onkeydown={onKey} />

{#if dev}
  <div class="bar" role="toolbar" aria-label="Variantes do protótipo">
    <button type="button" onclick={() => select(-1)} aria-label="Variante anterior">←</button>
    <span class="label"><strong>{current?.key}</strong> {current?.name}</span>
    <button type="button" onclick={() => select(1)} aria-label="Próxima variante">→</button>
    <span class="sep" aria-hidden="true"></span>
    {#each screens as screen (screen.href)}
      <a
        href={`${screen.href}?variant=${screen.chosen}`}
        aria-current={page.url.pathname === screen.href ? 'page' : undefined}>{screen.label}</a
      >
    {/each}
  </div>
{/if}

<style>
  .bar {
    position: fixed;
    bottom: 16px;
    left: 50%;
    z-index: 9999;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px;
    font:
      500 13px/1 'Inter Variable',
      system-ui,
      sans-serif;
    color: #fff;
    background: #111;
    border: 1px solid #333;
    border-radius: 999px;
    box-shadow: 0 10px 30px rgb(0 0 0 / 0.35);
    transform: translateX(-50%);
  }

  button,
  a {
    display: grid;
    place-items: center;
    min-width: 32px;
    height: 32px;
    padding: 0 10px;
    color: inherit;
    text-decoration: none;
    background: transparent;
    border: 0;
    border-radius: 999px;
    cursor: pointer;
  }

  button:hover,
  a:hover {
    background: #2a2a2a;
  }

  a[aria-current='page'] {
    color: #111;
    background: #fff;
  }

  .label {
    min-width: 170px;
    text-align: center;
    color: #ccc;
  }

  .label strong {
    color: #fff;
    margin-right: 4px;
  }

  .sep {
    width: 1px;
    height: 20px;
    margin: 0 4px;
    background: #333;
  }
</style>
