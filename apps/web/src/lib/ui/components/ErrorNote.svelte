<!-- Inline error or warning, e.g. under an input. -->
<script lang="ts">
  import Icon from './Icon.svelte';

  interface Props {
    message: string;
    tone?: 'danger' | 'warning';
  }

  let { message, tone = 'danger' }: Props = $props();
</script>

<p class="note {tone}" role="status">
  <Icon name="alert" size={14} />
  {message}
</p>

<style>
  .note {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
    margin: var(--space-2) 0 0;
    font-size: var(--text-xs);
  }

  .note :global(svg) {
    flex: none;
    margin-top: 1px;
  }

  .danger {
    color: var(--color-danger);
  }

  .warning {
    color: var(--color-warning);
  }
</style>
