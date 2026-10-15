<!--
  PROTOTYPE — live graph: the tangent sweeps along f and f′ is traced (dashed).
  Drag to take control. Colors come from --g-* custom properties.
-->
<script lang="ts">
  import { onMount, untrack } from 'svelte';

  interface Props {
    f: (x: number) => number;
    df: (x: number) => number;
    xMin?: number;
    xMax?: number;
    yMin?: number;
    yMax?: number;
    autoplay?: boolean;
    showDerivative?: boolean;
    readout?: boolean;
    x0?: number;
  }

  let {
    f,
    df,
    xMin = -3,
    xMax = 3,
    yMin = -4,
    yMax = 6,
    autoplay = true,
    showDerivative = true,
    readout = true,
    x0 = $bindable(-2.2),
  }: Props = $props();

  const W = 400;
  const H = 260;
  const sx = (x: number) => ((x - xMin) / (xMax - xMin)) * W;
  const sy = (y: number) => H - ((y - yMin) / (yMax - yMin)) * H;

  function path(fn: (x: number) => number, until = xMax): string {
    const steps = 160;
    let d = '';
    for (let i = 0; i <= steps; i++) {
      const x = xMin + ((until - xMin) * i) / steps;
      const y = Math.min(yMax + 5, Math.max(yMin - 5, fn(x)));
      d += `${i === 0 ? 'M' : 'L'}${sx(x).toFixed(1)},${sy(y).toFixed(1)}`;
    }
    return d;
  }

  const curve = $derived(path(f));
  const trace = $derived(path(df, x0));
  const slope = $derived(df(x0));
  const y0 = $derived(f(x0));
  const tangent = $derived({
    x1: sx(xMin),
    y1: sy(y0 + slope * (xMin - x0)),
    x2: sx(xMax),
    y2: sy(y0 + slope * (xMax - x0)),
  });

  let playing = $state(untrack(() => autoplay));
  let svg: SVGSVGElement | undefined = $state();

  onMount(() => {
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) playing = false;
    let frame = 0;
    const start = performance.now();
    const tick = (now: number) => {
      if (playing) {
        const t = ((now - start) / 6000) % 1;
        x0 = xMin * 0.8 + (xMax - xMin) * 0.8 * (0.5 - 0.5 * Math.cos(t * Math.PI * 2));
      }
      frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  });

  function drag(event: PointerEvent): void {
    if (!svg || (event.type === 'pointermove' && event.buttons === 0)) return;
    playing = false;
    const box = svg.getBoundingClientRect();
    x0 = xMin + ((event.clientX - box.left) / box.width) * (xMax - xMin);
  }

  const fmt = (n: number) => n.toFixed(2).replace('.', ',').replace('-', '−');
</script>

<figure class="graph">
  <svg
    bind:this={svg}
    viewBox={`0 0 ${W} ${H}`}
    role="img"
    aria-label={`Gráfico de f com a reta tangente em x = ${fmt(x0)}`}
    onpointerdown={drag}
    onpointermove={drag}
  >
    {#each [-2, -1, 0, 1, 2] as gx (gx)}
      <line class="grid" x1={sx(gx)} x2={sx(gx)} y1="0" y2={H} />
    {/each}
    <line class="axis" x1="0" x2={W} y1={sy(0)} y2={sy(0)} />
    <line class="axis" x1={sx(0)} x2={sx(0)} y1="0" y2={H} />
    {#if showDerivative}<path class="deriv" d={trace} />{/if}
    <path class="curve" d={curve} />
    <line class="tangent" {...tangent} />
    <circle class="point" cx={sx(x0)} cy={sy(y0)} r="6" />
  </svg>
  {#if readout}
    <figcaption>
      <span>x₀ = <b>{fmt(x0)}</b></span>
      <span>f′(x₀) = <b class:zero={Math.abs(slope) < 0.15}>{fmt(slope)}</b></span>
      {#if !playing}
        <button type="button" onclick={() => (playing = true)}>▶ animar</button>
      {/if}
    </figcaption>
  {/if}
</figure>

<style>
  .graph {
    margin: 0;
    display: grid;
    gap: 8px;
  }

  svg {
    width: 100%;
    height: auto;
    touch-action: none;
    cursor: ew-resize;
    border-radius: inherit;
  }

  .grid {
    stroke: var(--g-grid, #eee);
    stroke-width: 1;
  }

  .axis {
    stroke: var(--g-axis, #bbb);
    stroke-width: 1;
  }

  .curve {
    fill: none;
    stroke: var(--g-curve, #18181b);
    stroke-width: 2.5;
  }

  .deriv {
    fill: none;
    stroke: var(--g-deriv, #e11d48);
    stroke-width: 2;
    stroke-dasharray: 6 5;
  }

  .tangent {
    stroke: var(--g-tangent, #16a34a);
    stroke-width: 2;
  }

  .point {
    fill: var(--g-point, #4f46e5);
    stroke: var(--g-bg, #fff);
    stroke-width: 3;
  }

  figcaption {
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
    align-items: center;
    font:
      500 13px/1.2 'JetBrains Mono Variable',
      monospace;
    color: var(--g-muted, #71717a);
  }

  b {
    color: var(--g-text, #18181b);
    font-weight: 600;
  }

  b.zero {
    color: var(--g-tangent, #16a34a);
  }

  button {
    font: inherit;
    color: inherit;
    background: none;
    border: 1px solid currentColor;
    border-radius: 999px;
    padding: 4px 10px;
    cursor: pointer;
  }
</style>
