<!--
  PROTOTYPE — line chart that morphs between ranges, with a gliding crosshair and a
  dashed "previous period" series (inspired by uiarc line-chart).
-->
<script lang="ts">
  import { Tween } from 'svelte/motion';
  import { cubicOut } from 'svelte/easing';

  interface Props {
    current: readonly number[];
    previous: readonly number[];
    label: string;
    height?: number;
  }

  let { current, previous, label, height = 160 }: Props = $props();

  const N = 40;
  const W = 400;

  function resample(values: readonly number[]): number[] {
    return Array.from({ length: N }, (_, i) => {
      const t = (i / (N - 1)) * (values.length - 1);
      const lo = Math.floor(t);
      const hi = Math.min(values.length - 1, lo + 1);
      return (values[lo] ?? 0) + ((values[hi] ?? 0) - (values[lo] ?? 0)) * (t - lo);
    });
  }

  const cur = new Tween(resample([]), { duration: 650, easing: cubicOut });
  const prev = new Tween(resample([]), { duration: 650, easing: cubicOut });

  $effect(() => {
    cur.target = resample(current);
    prev.target = resample(previous);
  });

  const max = $derived(Math.max(1, ...current, ...previous) * 1.15);
  const x = (i: number) => (i / (N - 1)) * W;
  const y = (v: number) => height - (v / max) * height;

  function line(values: number[]): string {
    return values.map((v, i) => `${i ? 'L' : 'M'}${x(i).toFixed(1)},${y(v).toFixed(1)}`).join('');
  }

  const curPath = $derived(line(cur.current));
  const area = $derived(`${curPath}L${W},${height}L0,${height}Z`);

  let hover: number | null = $state(null);
  let svg: SVGSVGElement | undefined = $state();

  function move(event: PointerEvent): void {
    if (!svg) return;
    const box = svg.getBoundingClientRect();
    hover = Math.round(((event.clientX - box.left) / box.width) * (N - 1));
  }

  const hoverIndex = $derived(hover ?? N - 1);
  const sourceIndex = $derived(Math.round((hoverIndex / (N - 1)) * (current.length - 1)));
</script>

<div class="chart">
  <div class="readout">
    <span class="dot cur"></span> Este período <b>{current[sourceIndex] ?? 0}</b>
    <span class="dot prev"></span> Anterior <b>{previous[sourceIndex] ?? 0}</b>
    <span class="when"
      >{sourceIndex === current.length - 1 ? 'hoje' : `dia ${sourceIndex + 1}`}</span
    >
  </div>
  <svg
    bind:this={svg}
    viewBox={`0 0 ${W} ${height}`}
    preserveAspectRatio="none"
    role="img"
    aria-label={`${label}: último valor ${current.at(-1) ?? 0}, anterior ${previous.at(-1) ?? 0}`}
    onpointermove={move}
    onpointerleave={() => (hover = null)}
  >
    <defs>
      <linearGradient id="lc-fill" x1="0" x2="0" y1="0" y2="1">
        <stop offset="0" stop-color="var(--lc-color, #4f46e5)" stop-opacity="0.22" />
        <stop offset="1" stop-color="var(--lc-color, #4f46e5)" stop-opacity="0" />
      </linearGradient>
    </defs>
    <path d={area} fill="url(#lc-fill)" />
    <path class="prev" d={line(prev.current)} />
    <path class="cur" d={curPath} />
    <line class="cross" x1={x(hoverIndex)} x2={x(hoverIndex)} y1="0" y2={height} />
    <circle class="pt" cx={x(hoverIndex)} cy={y(cur.current[hoverIndex] ?? 0)} r="4" />
  </svg>
</div>

<style>
  .chart {
    display: grid;
    gap: 10px;
  }

  .readout {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 6px 10px;
    font-size: 12px;
    color: var(--lc-muted, #71717a);
  }

  .readout b {
    margin-right: 8px;
    color: var(--lc-text, #18181b);
    font-variant-numeric: tabular-nums;
  }

  .when {
    margin-left: auto;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .dot.cur {
    background: var(--lc-color, #4f46e5);
  }

  .dot.prev {
    border: 1.5px dashed var(--lc-muted, #71717a);
  }

  svg {
    width: 100%;
    height: auto;
    overflow: visible;
  }

  path.cur {
    fill: none;
    stroke: var(--lc-color, #4f46e5);
    stroke-width: 2.25;
    vector-effect: non-scaling-stroke;
  }

  path.prev {
    fill: none;
    stroke: var(--lc-muted, #a1a1aa);
    stroke-width: 1.5;
    stroke-dasharray: 5 5;
    vector-effect: non-scaling-stroke;
  }

  .cross {
    stroke: var(--lc-muted, #a1a1aa);
    stroke-width: 1;
    vector-effect: non-scaling-stroke;
    transition:
      x1 120ms,
      x2 120ms;
  }

  .pt {
    fill: var(--lc-color, #4f46e5);
    stroke: var(--lc-bg, #fff);
    stroke-width: 2;
  }
</style>
