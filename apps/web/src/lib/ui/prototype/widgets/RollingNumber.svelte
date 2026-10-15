<!-- PROTOTYPE — digits roll to the new value (uiarc number-field / donut center). -->
<script lang="ts">
  interface Props {
    value: number | string;
  }

  let { value }: Props = $props();

  const chars = $derived(String(value).split(''));
  const DIGITS = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
</script>

<span class="roll" aria-label={String(value)}>
  {#each chars as char, i (`${chars.length}-${i}`)}
    {#if DIGITS.includes(char)}
      <span class="col" aria-hidden="true">
        <span class="strip" style:transform={`translateY(-${Number(char) * 10}%)`}>
          {#each DIGITS as d (d)}<span>{d}</span>{/each}
        </span>
      </span>
    {:else}
      <span aria-hidden="true">{char}</span>
    {/if}
  {/each}
</span>

<style>
  .roll {
    display: inline-flex;
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }

  .col {
    display: inline-block;
    height: 1em;
    overflow: hidden;
  }

  .strip {
    display: flex;
    flex-direction: column;
    transition: transform 520ms cubic-bezier(0.2, 0.9, 0.1, 1);
  }

  .strip span {
    height: 1em;
  }

  @media (prefers-reduced-motion: reduce) {
    .strip {
      transition: none;
    }
  }
</style>
