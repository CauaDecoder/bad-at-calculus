<script lang="ts">
  import { onMount } from 'svelte';
  import { VectorLab } from '$lib/application/vector-lab.svelte';
  import { useEngines } from '$lib/composition/engines';
  import type { VectorInput } from '$lib/domain/vectors';
  import Card from '$lib/ui/components/Card.svelte';
  import ErrorNote from '$lib/ui/components/ErrorNote.svelte';
  import ExampleChips from '$lib/ui/components/ExampleChips.svelte';
  import Switch from '$lib/ui/components/Switch.svelte';
  import VectorInputField from '$lib/ui/components/VectorInput.svelte';
  import { describeEngineError } from '$lib/ui/copy/engine-messages';
  import SpaceCanvas from '$lib/ui/graph3d/SpaceCanvas.svelte';
  import TopicPage from '$lib/ui/layout/TopicPage.svelte';
  import Workspace from '$lib/ui/layout/Workspace.svelte';
  import { useTheme } from '$lib/ui/theme-context';
  import { vectorItems, type VectorSceneOptions } from '$lib/ui/vectors/scenes';
  import VectorResults from '$lib/ui/vectors/VectorResults.svelte';

  const theme = useTheme();
  const lab = new VectorLab(useEngines().vectors);

  let show = $state<VectorSceneOptions>({ sum: true, cross: true, projection: false });

  interface VectorExample {
    label: string;
    u: VectorInput;
    v: VectorInput;
    w: VectorInput | null;
  }

  const EXAMPLES: VectorExample[] = [
    { label: 'i × j = k', u: ['1', '0', '0'], v: ['0', '1', '0'], w: null },
    { label: 'ortogonais', u: ['2', '1', '-1'], v: ['1', '-1', '1'], w: null },
    { label: 'paralelos', u: ['1', '-2', '3'], v: ['-2', '4', '-6'], w: null },
    { label: 'ângulo de 60°', u: ['1', '0', '0'], v: ['1/2', 'sqrt(3)/2', '0'], w: null },
    { label: 'no plano (2D)', u: ['3', '1', '0'], v: ['1', '2', '0'], w: null },
    { label: 'produto misto', u: ['1', '2', '0'], v: ['0', '1', '3'], w: ['2', '0', '1'] },
    { label: 'coplanares', u: ['1', '0', '1'], v: ['0', '1', '1'], w: ['1', '1', '2'] },
  ];

  const LEGEND = [
    { key: 'sum', name: 'u + v e paralelogramo', color: 'var(--color-vector-extra)' },
    { key: 'cross', name: 'u × v', color: 'var(--color-vector-result)' },
    { key: 'projection', name: 'Projeção de u sobre v', color: 'var(--color-vector-w)' },
  ] as const;

  const items = $derived(lab.report ? vectorItems(lab.report, show) : []);
  const invalidField = $derived(lab.error?.field ?? null);

  onMount(() => {
    void lab.refresh();
  });
</script>

<TopicPage
  module="Vetores e Geometria Analítica"
  title="Operações com vetores"
  description="Digite u e v (frações e raízes são aceitas). Cada operação aparece resolvida com os seus números."
>
  <Workspace>
    {#snippet canvas()}
      <SpaceCanvas
        {items}
        theme={theme.theme}
        description="Vetores u e v no espaço, com u + v e u × v"
      />
    {/snippet}

    {#snippet inspector()}
      <Card title="Vetores">
        <div class="vectors">
          <VectorInputField
            name="u"
            field="u"
            color="var(--color-vector-u)"
            value={lab.u}
            {invalidField}
            oninput={(index, value) => lab.setComponent('u', index, value)}
          />
          <VectorInputField
            name="v"
            field="v"
            color="var(--color-vector-v)"
            value={lab.v}
            {invalidField}
            oninput={(index, value) => lab.setComponent('v', index, value)}
          />
          {#if lab.useW}
            <VectorInputField
              name="w"
              field="w"
              color="var(--color-vector-w)"
              value={lab.w}
              {invalidField}
              oninput={(index, value) => lab.setComponent('w', index, value)}
            />
          {/if}
        </div>
        <div class="toggle">
          <span>Terceiro vetor w (produto misto)</span>
          <Switch
            label="Usar o vetor w"
            checked={lab.useW}
            onchange={(useW) => lab.setUseW(useW)}
          />
        </div>
        {#if lab.error}
          <ErrorNote message={describeEngineError(lab.error)} />
        {/if}
        <div class="examples">
          <ExampleChips
            examples={EXAMPLES}
            onpick={(example) => lab.loadExample(example.u, example.v, example.w)}
          />
        </div>
      </Card>

      <Card title="No gráfico">
        <ul class="legend">
          {#each LEGEND as item (item.key)}
            <li>
              <span class="swatch" style:--swatch={item.color}></span>
              <span class="name">{item.name}</span>
              <Switch
                label={item.name}
                checked={show[item.key]}
                onchange={(visible) => (show[item.key] = visible)}
              />
            </li>
          {/each}
        </ul>
      </Card>

      {#if lab.report}
        <VectorResults report={lab.report} />
      {/if}
    {/snippet}
  </Workspace>
</TopicPage>

<style>
  .vectors {
    display: grid;
    gap: var(--space-2);
  }

  .toggle {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: var(--space-3);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .examples {
    margin-top: var(--space-3);
  }

  .legend {
    display: grid;
    gap: var(--space-1);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .legend li {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    min-height: 30px;
  }

  .swatch {
    width: 14px;
    border-top: 2.5px solid var(--swatch);
  }

  .name {
    flex: 1;
  }
</style>
