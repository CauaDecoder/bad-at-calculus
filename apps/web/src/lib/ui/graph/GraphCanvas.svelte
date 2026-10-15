<!--
  Interactive 2D plotting surface (Canvas 2D).
  - drag elsewhere: pans            - wheel / + / −: zooms          - 0: resets the view
  With a `handle` (e.g. the point x₀):
  - drag the handle or click: moves it     - ← →: nudges it (Shift = ×10)
  Receives already-computed layers; never talks to the engine directly.
-->
<script lang="ts">
  import { niceGridStep } from '$lib/domain/axis-ticks';
  import {
    panByPixels,
    screenToWorld,
    visibleBounds,
    worldToScreen,
    zoomAt,
    type ScreenPoint,
    type Viewport,
  } from '$lib/domain/viewport';
  import IconButton from '$lib/ui/components/IconButton.svelte';
  import { MIN_GRID_SPACING_PX, paintGraph, type GraphScene } from './graph-painter';
  import { readPlotColors } from './plot-colors';

  interface Handle {
    readonly x: number;
    readonly y: number;
    readonly label: string;
    readonly valueText: string;
  }

  interface Props {
    scene: GraphScene;
    /** Changing it repaints with the new theme colors. */
    theme: string;
    description: string;
    hint: string;
    handle?: Handle | null;
    onHandleMove?: (x: number) => void;
    onViewportChange: (viewport: Viewport) => void;
    onCanvasResize: (widthPx: number, heightPx: number) => void;
    onResetView: () => void;
  }

  let {
    scene,
    theme,
    description,
    hint,
    handle = null,
    onHandleMove,
    onViewportChange,
    onCanvasResize,
    onResetView,
  }: Props = $props();

  const HANDLE_HIT_RADIUS_PX = 14;
  const CLICK_TOLERANCE_PX = 4;
  const WHEEL_ZOOM_SPEED = 0.0015;
  const BUTTON_ZOOM_FACTOR = 1.5;
  const NUDGE_GRID_FRACTION = 0.1;
  const FAST_NUDGE_MULTIPLIER = 10;

  interface Drag {
    mode: 'handle' | 'pan';
    start: ScreenPoint;
    last: ScreenPoint;
    moved: boolean;
  }

  let container: HTMLDivElement;
  let surface: HTMLDivElement;
  let canvas: HTMLCanvasElement;
  let drag: Drag | null = null;
  let cursor = $state('grab');
  let animationFrame = 0;
  const bounds = $derived(visibleBounds(scene.viewport));
  const accessibility = $derived(
    handle
      ? {
          role: 'slider',
          tabindex: 0,
          'aria-roledescription': 'gráfico interativo',
          'aria-label': handle.label,
          'aria-valuemin': bounds.xMin,
          'aria-valuemax': bounds.xMax,
          'aria-valuenow': handle.x,
          'aria-valuetext': `${handle.valueText}. ${description}`,
          onkeydown: handleKeyDown,
        }
      : { role: 'img', 'aria-label': description },
  );

  $effect(() => {
    void scene;
    void theme;
    scheduleRepaint();
  });

  $effect(() => {
    const observer = new ResizeObserver(([entry]) => {
      if (!entry) return;
      const { width, height } = entry.contentRect;
      resizeCanvas(width, height);
      onCanvasResize(width, height);
    });
    observer.observe(container);
    surface.addEventListener('wheel', handleWheel, { passive: false });
    return () => {
      observer.disconnect();
      surface.removeEventListener('wheel', handleWheel);
      cancelAnimationFrame(animationFrame);
    };
  });

  function resizeCanvas(widthPx: number, heightPx: number) {
    const pixelRatio = window.devicePixelRatio || 1;
    canvas.width = Math.round(widthPx * pixelRatio);
    canvas.height = Math.round(heightPx * pixelRatio);
    scheduleRepaint();
  }

  function scheduleRepaint() {
    if (animationFrame !== 0 || !canvas) return;
    animationFrame = requestAnimationFrame(() => {
      animationFrame = 0;
      repaint();
    });
  }

  function repaint() {
    const context = canvas.getContext('2d');
    if (!context) return;
    const pixelRatio = canvas.width / Math.max(scene.viewport.widthPx, 1);
    context.setTransform(pixelRatio, 0, 0, pixelRatio, 0, 0);
    paintGraph(context, scene, readPlotColors(container));
  }

  function localPoint(event: MouseEvent): ScreenPoint {
    const rect = canvas.getBoundingClientRect();
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }

  function isOverHandle(point: ScreenPoint): boolean {
    if (!handle) return false;
    const target = worldToScreen(scene.viewport, handle.x, handle.y);
    return Math.hypot(point.x - target.x, point.y - target.y) <= HANDLE_HIT_RADIUS_PX;
  }

  function worldXAt(point: ScreenPoint): number {
    return screenToWorld(scene.viewport, point.x, point.y).x;
  }

  function idleCursor(point: ScreenPoint): string {
    return isOverHandle(point) ? 'ew-resize' : 'grab';
  }

  function handlePointerDown(event: PointerEvent) {
    if (event.button !== 0) return;
    const point = localPoint(event);
    drag = {
      mode: isOverHandle(point) ? 'handle' : 'pan',
      start: point,
      last: point,
      moved: false,
    };
    cursor = 'grabbing';
    surface.setPointerCapture(event.pointerId);
  }

  function handlePointerMove(event: PointerEvent) {
    const point = localPoint(event);
    if (!drag) {
      cursor = idleCursor(point);
      return;
    }
    drag.moved ||= Math.hypot(point.x - drag.start.x, point.y - drag.start.y) > CLICK_TOLERANCE_PX;
    if (drag.mode === 'handle') {
      onHandleMove?.(worldXAt(point));
    } else {
      onViewportChange(panByPixels(scene.viewport, point.x - drag.last.x, point.y - drag.last.y));
    }
    drag.last = point;
  }

  function handlePointerUp(event: PointerEvent) {
    const wasClick = drag?.mode === 'pan' && !drag.moved;
    if (wasClick) onHandleMove?.(worldXAt(localPoint(event)));
    drag = null;
    cursor = idleCursor(localPoint(event));
  }

  function handleWheel(event: WheelEvent) {
    event.preventDefault();
    const factor = Math.exp(-event.deltaY * WHEEL_ZOOM_SPEED);
    onViewportChange(zoomAt(scene.viewport, factor, localPoint(event)));
  }

  function handleKeyDown(event: KeyboardEvent) {
    const gridStep = niceGridStep(scene.viewport.pixelsPerUnit, MIN_GRID_SPACING_PX);
    const nudge = gridStep * NUDGE_GRID_FRACTION * (event.shiftKey ? FAST_NUDGE_MULTIPLIER : 1);
    const x = handle?.x ?? 0;
    const actions: Record<string, () => void> = {
      ArrowLeft: () => onHandleMove?.(x - nudge),
      ArrowRight: () => onHandleMove?.(x + nudge),
      '+': zoomIn,
      '=': zoomIn,
      '-': zoomOut,
      '0': onResetView,
    };
    const action = actions[event.key];
    if (!action) return;
    event.preventDefault();
    action();
  }

  function zoomIn() {
    onViewportChange(zoomAt(scene.viewport, BUTTON_ZOOM_FACTOR));
  }

  function zoomOut() {
    onViewportChange(zoomAt(scene.viewport, 1 / BUTTON_ZOOM_FACTOR));
  }
</script>

<div class="graph" bind:this={container}>
  <!-- One element whose role depends on the handle: a slider for x₀, or an image. -->
  <div
    bind:this={surface}
    class="surface"
    style:cursor
    {...accessibility}
    onpointerdown={handlePointerDown}
    onpointermove={handlePointerMove}
    onpointerup={handlePointerUp}
    onpointercancel={() => (drag = null)}
  >
    <canvas bind:this={canvas} aria-hidden="true"></canvas>
  </div>

  <div class="hint" aria-hidden="true">{hint}</div>

  <div class="zoom-controls">
    <IconButton icon="plus" label="Aproximar (+)" onclick={zoomIn} />
    <IconButton icon="minus" label="Afastar (−)" onclick={zoomOut} />
    <IconButton icon="scan" label="Enquadrar (0)" onclick={onResetView} />
  </div>
</div>

<style>
  .graph {
    position: relative;
    height: 100%;
    min-height: 320px;
    overflow: hidden;
    background: var(--color-plot-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .surface {
    position: absolute;
    inset: 0;
    touch-action: none;
  }

  canvas {
    display: block;
    width: 100%;
    height: 100%;
  }

  .surface:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: -2px;
    border-radius: var(--radius-lg);
  }

  .hint {
    position: absolute;
    left: var(--space-3);
    bottom: var(--space-3);
    max-width: calc(100% - 140px);
    padding: var(--space-1) var(--space-3);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-full);
    pointer-events: none;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .zoom-controls {
    position: absolute;
    right: var(--space-3);
    bottom: var(--space-3);
    display: flex;
    gap: 2px;
    padding: 2px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
  }
</style>
