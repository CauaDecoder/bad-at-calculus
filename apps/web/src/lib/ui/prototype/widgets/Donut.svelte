<!--
  PROTOTYPE — mastery donut: arcs morph between datasets and the active value rolls into
  the center (inspired by uiarc donut-chart).
-->
<script lang="ts">
  import { Tween } from 'svelte/motion';
  import { cubicOut } from 'svelte/easing';
  import { untrack } from 'svelte';
  import RollingNumber from './RollingNumber.svelte';

  interface Segment {
    readonly topic: string;
    readonly value: number;
    readonly color: string;
  }

  interface Props {
    segments: readonly Segment[];
    size?: number;
  }

  let { segments, size = 180 }: Props = $props();

  // Start from zeros of the same length: Tween cannot interpolate arrays of different sizes.
  const values = new Tween<number[]>(
    untrack(() => segments.map(() => 0)),
    { duration: 700, easing: cubicOut },
  );
  $effect(() => {
    values.target = segments.map((s) => s.value);
  });

  let active: number | null = $state(null);

  const R = 70;
  const C = 2 * Math.PI * R;
  const GAP = 0.012;
  // Each topic owns an equal slice; the filled part of the slice is its mastery.
  const slice = $derived(1 / Math.max(1, segments.length));
  const average = $derived(
    Math.round(segments.reduce((sum, s) => sum + s.value, 0) / Math.max(1, segments.length)),
  );
  const shown = $derived(active === null ? average : (segments[active]?.value ?? 0));
  const caption = $derived(active === null ? 'domínio médio' : (segments[active]?.topic ?? ''));
</script>

<div class="donut" style:--size={`${size}px`}>
  <svg viewBox="0 0 180 180" role="img" aria-label={`Domínio médio ${average}%`}>
    {#each segments as segment, i (segment.topic)}
      {@const start = i * slice}
      {@const filled = ((values.current[i] ?? 0) / 100) * (slice - GAP)}
      <circle
        class="track"
        cx="90"
        cy="90"
        r={R}
        stroke-dasharray={`${(slice - GAP) * C} ${C}`}
        stroke-dashoffset={-start * C}
      />
      <circle
        class="arc"
        class:dim={active !== null && active !== i}
        cx="90"
        cy="90"
        r={R}
        stroke={segment.color}
        stroke-dasharray={`${filled * C} ${C}`}
        stroke-dashoffset={-start * C}
        role="presentation"
        onpointerenter={() => (active = i)}
        onpointerleave={() => (active = null)}
      />
    {/each}
  </svg>
  <div class="center">
    <span class="value"><RollingNumber value={`${shown}%`} /></span>
    <span class="caption">{caption}</span>
  </div>
</div>

<style>
  .donut {
    position: relative;
    width: var(--size);
    height: var(--size);
  }

  svg {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
  }

  circle {
    fill: none;
    stroke-width: 18;
  }

  .track {
    stroke: var(--d-track, #f1f1f3);
  }

  .arc {
    stroke-linecap: round;
    transition: opacity 160ms;
    cursor: pointer;
  }

  .arc.dim {
    opacity: 0.3;
  }

  .center {
    position: absolute;
    inset: 0;
    display: grid;
    place-content: center;
    text-align: center;
    pointer-events: none;
  }

  .value {
    font-size: 30px;
    font-weight: 600;
    letter-spacing: -0.02em;
    color: var(--d-text, #18181b);
  }

  .caption {
    margin-top: 6px;
    font-size: 12px;
    color: var(--d-muted, #71717a);
  }
</style>
