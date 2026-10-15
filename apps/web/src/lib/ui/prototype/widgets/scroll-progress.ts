/**
 * PROTOTYPE — `use:scrollProgress={(p) => …}`: reports 0..1 as the element crosses the viewport.
 * 0 when its top enters the bottom of the screen; 1 when its top reaches 35% of the height.
 * Drives scroll-linked motion in Site D (things move *with* the page, not on a timer).
 */
export function scrollProgress(node: HTMLElement, onProgress: (progress: number) => void) {
  let callback = onProgress;

  function update(): void {
    const top = node.getBoundingClientRect().top;
    const vh = window.innerHeight;
    const raw = (vh * 0.95 - top) / (vh * 0.6);
    callback(Math.min(1, Math.max(0, raw)));
  }

  const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  if (reduced) {
    callback(1);
  } else {
    window.addEventListener('scroll', update, { passive: true });
    window.addEventListener('resize', update);
    requestAnimationFrame(update);
  }

  return {
    update(next: (progress: number) => void) {
      callback = next;
    },
    destroy() {
      window.removeEventListener('scroll', update);
      window.removeEventListener('resize', update);
    },
  };
}

/** Maps overall progress `p` to a sub-range [from, to] → 0..1 (for staggered pieces). */
export function segment(p: number, from: number, to: number): number {
  return Math.min(1, Math.max(0, (p - from) / (to - from)));
}
