<!--
  3D view for vectors, points and planes. Drag to rotate, wheel to zoom.
  The camera is view state only, so it lives here and not in the application layer.
-->
<script lang="ts">
  import {
    createDefaultCamera,
    fitCamera,
    orbit,
    resizeCamera,
    topViewCamera,
    zoomCamera,
    type Camera3D,
  } from '$lib/domain/camera3d';
  import type { Vector3 } from '$lib/domain/vectors';
  import IconButton from '$lib/ui/components/IconButton.svelte';
  import { paintSpace, readSpaceColors, type SpaceItem } from './space-painter';

  interface Props {
    items: readonly SpaceItem[];
    theme: string;
    description: string;
  }

  let { items, theme, description }: Props = $props();

  const WHEEL_ZOOM_SPEED = 0.0015;
  const BUTTON_ZOOM_FACTOR = 1.4;
  const MIN_EXTENT = 2;
  /** Refit when the scene outgrows the view, or becomes tiny inside it. */
  const OUTGROWN_RATIO = 0.95;
  const SHRUNK_RATIO = 0.25;

  let container: HTMLDivElement;
  let canvas: HTMLCanvasElement;
  let camera = $state.raw<Camera3D>(createDefaultCamera(600, 400));
  let hasSize = false;
  let dragging = $state<{ x: number; y: number } | null>(null);
  let animationFrame = 0;

  const points = $derived(collectPoints(items));
  const extent = $derived(
    Math.max(MIN_EXTENT, ...points.flatMap((point) => point.map((value) => Math.abs(value)))),
  );

  $effect(() => {
    void items;
    void theme;
    void camera;
    scheduleRepaint();
  });

  $effect(() => {
    const visibleExtent = Math.min(camera.widthPx, camera.heightPx) / 2 / camera.pixelsPerUnit;
    const needsFit =
      extent > visibleExtent * OUTGROWN_RATIO || extent < visibleExtent * SHRUNK_RATIO;
    if (hasSize && needsFit) camera = fitCamera(camera, points);
  });

  $effect(() => {
    const observer = new ResizeObserver(([entry]) => {
      if (!entry) return;
      const { width, height } = entry.contentRect;
      const pixelRatio = window.devicePixelRatio || 1;
      canvas.width = Math.round(width * pixelRatio);
      canvas.height = Math.round(height * pixelRatio);
      const resized = resizeCamera(camera, width, height);
      camera = hasSize ? resized : fitCamera(resized, points);
      hasSize = true;
    });
    observer.observe(container);
    container.addEventListener('wheel', handleWheel, { passive: false });
    return () => {
      observer.disconnect();
      container.removeEventListener('wheel', handleWheel);
      cancelAnimationFrame(animationFrame);
    };
  });

  function collectPoints(sceneItems: readonly SpaceItem[]): Vector3[] {
    return sceneItems.flatMap((item) => {
      switch (item.kind) {
        case 'arrow':
        case 'segment':
          return [item.from, item.to];
        case 'point':
          return [item.at];
        case 'polygon':
          return [...item.corners];
      }
    });
  }

  function scheduleRepaint() {
    if (animationFrame !== 0 || !canvas) return;
    animationFrame = requestAnimationFrame(() => {
      animationFrame = 0;
      const context = canvas.getContext('2d');
      if (!context) return;
      const pixelRatio = canvas.width / Math.max(camera.widthPx, 1);
      context.setTransform(pixelRatio, 0, 0, pixelRatio, 0, 0);
      paintSpace(context, { camera, items, extent }, readSpaceColors(container));
    });
  }

  function handlePointerDown(event: PointerEvent) {
    dragging = { x: event.clientX, y: event.clientY };
    (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
  }

  function handlePointerMove(event: PointerEvent) {
    if (!dragging) return;
    camera = orbit(camera, event.clientX - dragging.x, event.clientY - dragging.y);
    dragging = { x: event.clientX, y: event.clientY };
  }

  function handleWheel(event: WheelEvent) {
    event.preventDefault();
    camera = zoomCamera(camera, Math.exp(-event.deltaY * WHEEL_ZOOM_SPEED));
  }

  function showDefaultView() {
    camera = fitCamera(createDefaultCamera(camera.widthPx, camera.heightPx), points);
  }
</script>

<div class="space" bind:this={container}>
  <div
    class="surface"
    class:dragging={dragging !== null}
    role="img"
    aria-label={description}
    onpointerdown={handlePointerDown}
    onpointermove={handlePointerMove}
    onpointerup={() => (dragging = null)}
    onpointercancel={() => (dragging = null)}
  >
    <canvas bind:this={canvas} aria-hidden="true"></canvas>
  </div>

  <div class="hint" aria-hidden="true">Arraste para girar · role para zoom</div>

  <div class="controls">
    <button type="button" class="view-button" onclick={showDefaultView}>3D</button>
    <button type="button" class="view-button" onclick={() => (camera = topViewCamera(camera))}>
      Plano xy
    </button>
    <IconButton
      icon="plus"
      label="Aproximar"
      onclick={() => (camera = zoomCamera(camera, BUTTON_ZOOM_FACTOR))}
    />
    <IconButton
      icon="minus"
      label="Afastar"
      onclick={() => (camera = zoomCamera(camera, 1 / BUTTON_ZOOM_FACTOR))}
    />
    <IconButton
      icon="scan"
      label="Enquadrar"
      onclick={() => (camera = fitCamera(camera, points))}
    />
  </div>
</div>

<style>
  .space {
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
    cursor: grab;
    touch-action: none;
  }

  .surface.dragging {
    cursor: grabbing;
  }

  canvas {
    display: block;
    width: 100%;
    height: 100%;
  }

  .hint {
    position: absolute;
    left: var(--space-3);
    bottom: var(--space-3);
    padding: var(--space-1) var(--space-3);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-full);
    pointer-events: none;
  }

  .controls {
    position: absolute;
    right: var(--space-3);
    bottom: var(--space-3);
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 2px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
  }

  .view-button {
    height: 32px;
    padding: 0 var(--space-2);
    font: inherit;
    font-size: var(--text-xs);
    font-weight: 500;
    color: var(--color-text-muted);
    background: transparent;
    border: 0;
    border-radius: var(--radius-md);
    cursor: pointer;
  }

  .view-button:hover {
    color: var(--color-text);
    background: var(--color-surface-hover);
  }

  .view-button:focus-visible {
    outline: 2px solid var(--color-accent);
  }
</style>
