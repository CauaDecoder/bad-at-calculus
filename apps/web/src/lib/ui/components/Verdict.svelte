<!-- Yes/no answer with an icon, e.g. "✓ Contínua em x = 2" or "✗ Não são ortogonais". -->
<script lang="ts">
  import Icon from './Icon.svelte';

  interface Props {
    positive: boolean;
    text: string;
    detail?: string;
  }

  let { positive, text, detail }: Props = $props();
</script>

<div class="verdict" class:positive>
  <span class="icon"><Icon name={positive ? 'check' : 'cross'} size={14} /></span>
  <div>
    <p class="text">{text}</p>
    {#if detail}<p class="detail">{detail}</p>{/if}
  </div>
</div>

<style>
  .verdict {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    color: var(--color-text);
    background: color-mix(in srgb, var(--color-danger) 8%, transparent);
    border: 1px solid color-mix(in srgb, var(--color-danger) 30%, transparent);
    border-radius: var(--radius-md);
  }

  .verdict.positive {
    background: color-mix(in srgb, var(--color-success) 8%, transparent);
    border-color: color-mix(in srgb, var(--color-success) 30%, transparent);
  }

  .icon {
    display: grid;
    place-items: center;
    flex: none;
    width: 20px;
    height: 20px;
    margin-top: 1px;
    color: #fff;
    background: var(--color-danger);
    border-radius: 50%;
  }

  .positive .icon {
    background: var(--color-success);
  }

  .text {
    margin: 0;
    font-weight: 500;
  }

  .detail {
    margin: 2px 0 0;
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }
</style>
