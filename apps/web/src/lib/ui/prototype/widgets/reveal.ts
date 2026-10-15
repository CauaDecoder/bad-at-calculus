/**
 * PROTOTYPE — `use:reveal`: adds `is-visible` once the element scrolls into view.
 * Scroll/resize based (not IntersectionObserver) so it behaves the same in throttled panes.
 * The element should start hidden through CSS on `:not(.is-visible)`.
 */
export function reveal(node: HTMLElement, offset = 0.85) {
  function check(): void {
    if (node.getBoundingClientRect().top < window.innerHeight * offset) {
      node.classList.add('is-visible');
      stop();
    }
  }

  function stop(): void {
    window.removeEventListener('scroll', check);
    window.removeEventListener('resize', check);
  }

  if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
    node.classList.add('is-visible');
  } else {
    window.addEventListener('scroll', check, { passive: true });
    window.addEventListener('resize', check);
    requestAnimationFrame(check);
  }

  return { destroy: stop };
}
