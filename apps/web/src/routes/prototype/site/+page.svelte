<!--
  PROTOTYPE — three variants of the Site screen, switchable via ?variant=A|B|C.
  Design direction and evaluation notes: docs/references/design/README.md.
-->
<script lang="ts">
  import { page } from '$app/state';
  import PrototypeSwitcher from '$lib/ui/prototype/PrototypeSwitcher.svelte';
  import SiteA from '$lib/ui/prototype/site/SiteA.svelte';
  import SiteB from '$lib/ui/prototype/site/SiteB.svelte';
  import SiteC from '$lib/ui/prototype/site/SiteC.svelte';
  import SiteD from '$lib/ui/prototype/site/SiteD.svelte';

  const variants = [
    { key: 'A', name: 'Problema → prova → CTA' },
    { key: 'B', name: 'Ferramenta no herói' },
    { key: 'C', name: 'Narrativa por scroll' },
    { key: 'D', name: 'A + botões da B + scroll da C' },
  ];
  const variant = $derived(page.url.searchParams.get('variant') ?? 'D');
</script>

<svelte:head><title>Protótipo · Site {variant} — Vértice</title></svelte:head>

{#if variant === 'B'}
  <SiteB />
{:else if variant === 'D'}
  <SiteD />
{:else if variant === 'C'}
  <SiteC />
{:else}
  <SiteA />
{/if}

<PrototypeSwitcher {variants} defaultKey="D" />
