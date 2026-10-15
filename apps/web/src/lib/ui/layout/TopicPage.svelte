<!-- A topic page: header on top, body filling the remaining height. -->
<script lang="ts">
  import type { Snippet } from 'svelte';
  import PageHeader from './PageHeader.svelte';

  interface Props {
    module: string;
    title: string;
    description: string;
    children: Snippet;
  }

  let { module, title, description, children }: Props = $props();
</script>

<svelte:head>
  <title>{title} · Math View</title>
</svelte:head>

<div class="topic-page">
  <PageHeader {module} {title} {description} />
  <div class="body">{@render children()}</div>
</div>

<style>
  .topic-page {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .body {
    flex: 1;
    min-height: 0;
  }

  @media (max-width: 1100px) {
    .topic-page {
      height: auto;
    }
  }
</style>
