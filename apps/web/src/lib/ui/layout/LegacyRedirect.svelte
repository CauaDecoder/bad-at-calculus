<script lang="ts">
  import { onMount } from 'svelte';
  import { resolve } from '$app/paths';
  import type { RouteId } from '$app/types';
  import { NAVIGATION } from '$lib/ui/copy/navigation';

  let { destination }: { destination: RouteId } = $props();

  // Client-side replacement works in prerendered files and preserves opaque URL state.
  onMount(() => {
    window.location.replace(resolve(destination) + window.location.search + window.location.hash);
  });
</script>

<svelte:head>
  <title>{NAVIGATION.redirect} · {NAVIGATION.brand}</title>
</svelte:head>

<p>{NAVIGATION.redirect} <a href={resolve(destination)}>{NAVIGATION.continue}</a></p>
