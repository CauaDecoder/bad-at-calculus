<!-- Three components of a vector or point, e.g. u = (1, 2, 2). -->
<script lang="ts">
  import { AXIS_NAMES, type VectorInput } from '$lib/domain/vectors';
  import NumberField from './NumberField.svelte';

  interface Props {
    name: string;
    /** Legend color, e.g. `var(--color-vector-u)`. */
    color: string;
    value: VectorInput;
    /** Id prefix used by engine errors (`u` → `u.x`, `u.y`, `u.z`). */
    field: string;
    invalidField?: string | null;
    oninput: (index: number, value: string) => void;
  }

  let { name, color, value, field, invalidField = null, oninput }: Props = $props();
</script>

<div class="vector-input">
  <span class="name" style:color>{name}</span>
  <span class="bracket" aria-hidden="true">(</span>
  {#each value as component, index (index)}
    <NumberField
      value={component}
      label={`${name}, componente ${AXIS_NAMES[index]}`}
      invalid={invalidField === `${field}.${AXIS_NAMES[index]}`}
      oninput={(next) => oninput(index, next)}
    />
    {#if index < 2}<span class="comma" aria-hidden="true">,</span>{/if}
  {/each}
  <span class="bracket" aria-hidden="true">)</span>
</div>

<style>
  .vector-input {
    display: grid;
    grid-template-columns: 28px auto 1fr auto 1fr auto 1fr auto;
    align-items: center;
    gap: 2px;
  }

  .name {
    font-family: var(--font-mono);
    font-weight: 600;
  }

  .bracket,
  .comma {
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }
</style>
