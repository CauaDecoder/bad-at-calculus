<script lang="ts">
  import { onMount } from 'svelte';
  import { PlaneLab, type PointName } from '$lib/application/vector-lab.svelte';
  import { useEngines } from '$lib/composition/engines';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import VectorInputField from '$lib/ui/components/VectorInput.svelte';
  import { describeEngineError } from '$lib/ui/copy/engine-messages';
  import SpaceCanvas from '$lib/ui/graph3d/SpaceCanvas.svelte';
  import TopicPage from '$lib/ui/layout/TopicPage.svelte';
  import Workspace from '$lib/ui/layout/Workspace.svelte';
  import { useTheme } from '$lib/ui/theme-context';
  import PlaneResults from '$lib/ui/vectors/PlaneResults.svelte';
  import { planeItems } from '$lib/ui/vectors/scenes';

  const theme = useTheme();
  const lab = new PlaneLab(useEngines().vectors);

  const POINTS: { name: PointName; label: string; color: string }[] = [
    { name: 'a', label: 'A', color: 'var(--color-vector-u)' },
    { name: 'b', label: 'B', color: 'var(--color-vector-u)' },
    { name: 'c', label: 'C', color: 'var(--color-vector-u)' },
    { name: 'p', label: 'P', color: 'var(--color-vector-w)' },
  ];

  const items = $derived(lab.report ? planeItems(lab.report) : []);

  onMount(() => {
    void lab.refresh();
  });
</script>

<TopicPage
  module="Vetores e Geometria Analítica"
  title="Retas e planos"
  description="O plano que passa por A, B e C, a reta AB e as distâncias de um ponto P até cada um."
>
  <Workspace>
    {#snippet canvas()}
      <SpaceCanvas
        {items}
        theme={theme.theme}
        description="Plano por A, B e C, reta AB e ponto P"
      />
    {/snippet}

    {#snippet inspector()}
      <Card title="Pontos">
        <div class="points">
          {#each POINTS as point (point.name)}
            <VectorInputField
              name={point.label}
              field={point.name}
              color={point.color}
              value={lab[point.name]}
              invalidField={lab.error?.field ?? null}
              oninput={(index, value) => lab.setComponent(point.name, index, value)}
            />
          {/each}
        </div>
        {#if lab.error}
          <ErrorNote message={describeEngineError(lab.error)} />
        {/if}
      </Card>

      {#if lab.report}
        <PlaneResults report={lab.report} />
      {/if}
    {/snippet}
  </Workspace>
</TopicPage>

<style>
  .points {
    display: grid;
    gap: var(--space-2);
  }
</style>
