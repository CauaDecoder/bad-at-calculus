<!--
  PROTOTYPE — three variants of the Exercício screen, switchable via ?variant=A|B|C.
  Design direction and evaluation notes: docs/references/design/README.md.
-->
<script lang="ts">
  import { page } from '$app/state';
  import PrototypeSwitcher from '$lib/ui/prototype/PrototypeSwitcher.svelte';
  import ExercicioA from '$lib/ui/prototype/exercicio/ExercicioA.svelte';
  import ExercicioB from '$lib/ui/prototype/exercicio/ExercicioB.svelte';
  import ExercicioC from '$lib/ui/prototype/exercicio/ExercicioC.svelte';
  import ExercicioD from '$lib/ui/prototype/exercicio/ExercicioD.svelte';

  const variants = [
    { key: 'A', name: 'Laboratório dividido' },
    { key: 'B', name: 'Cartão em foco' },
    { key: 'C', name: 'Caderno' },
    { key: 'D', name: 'A + barra de progresso da B' },
  ];
  const variant = $derived(page.url.searchParams.get('variant') ?? 'D');
</script>

<svelte:head><title>Protótipo · Exercício {variant} — Vértice</title></svelte:head>

{#if variant === 'B'}
  <ExercicioB />
{:else if variant === 'D'}
  <ExercicioD />
{:else if variant === 'C'}
  <ExercicioC />
{:else}
  <ExercicioA />
{/if}

<PrototypeSwitcher {variants} defaultKey="D" />
