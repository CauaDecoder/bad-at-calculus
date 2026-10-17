<!--
  PROTOTYPE — three variants of the app home, switchable via ?variant=A|B|C.
  Design direction and evaluation notes: docs/references/design/README.md.
-->
<script lang="ts">
  import { page } from '$app/state';
  import PrototypeSwitcher from '$lib/ui/prototype/PrototypeSwitcher.svelte';
  import InicioA from '$lib/ui/prototype/inicio/InicioA.svelte';
  import InicioB from '$lib/ui/prototype/inicio/InicioB.svelte';
  import InicioC from '$lib/ui/prototype/inicio/InicioC.svelte';

  const variants = [
    { key: 'A', name: 'Para hoje' },
    { key: 'B', name: 'Prompt primeiro' },
    { key: 'C', name: 'Painel de progresso' },
  ];
  const variant = $derived(page.url.searchParams.get('variant') ?? 'A');
</script>

<svelte:head><title>Protótipo · Início {variant} — Vértice</title></svelte:head>

{#if variant === 'B'}
  <InicioB />
{:else if variant === 'C'}
  <InicioC />
{:else}
  <InicioA />
{/if}

<PrototypeSwitcher {variants} />
