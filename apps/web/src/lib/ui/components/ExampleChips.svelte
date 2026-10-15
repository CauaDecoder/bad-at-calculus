<!-- Ready-made examples (many from the exercise lists) that fill the inputs on click. -->
<script lang="ts" generics="Example extends { label: string }">
  interface Props {
    examples: readonly Example[];
    onpick: (example: Example) => void;
  }

  let { examples, onpick }: Props = $props();
</script>

<div class="examples">
  <span class="title">Exemplos</span>
  <div class="chips">
    {#each examples as example (example.label)}
      <button type="button" class="chip" onclick={() => onpick(example)}>{example.label}</button>
    {/each}
  </div>
</div>

<style>
  .examples {
    display: grid;
    gap: var(--space-2);
  }

  .title {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .chips {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
  }

  .chip {
    padding: 3px var(--space-2);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-text);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-full);
    cursor: pointer;
    transition:
      border-color var(--duration-fast) var(--easing-standard),
      background var(--duration-fast) var(--easing-standard);
  }

  .chip:hover {
    border-color: var(--color-accent);
    background: var(--color-accent-soft);
  }

  .chip:focus-visible {
    outline: 2px solid var(--color-accent);
  }
</style>
