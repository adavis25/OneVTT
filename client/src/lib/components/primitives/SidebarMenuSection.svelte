<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    label?: string;
    class?: string;
    children?: Snippet;
  }

  let { label, class: className, children }: Props = $props();
</script>

<div class={cn('space-y-0.5', className)}>
  {#if label}
    <p class="px-2 py-1 text-xs font-medium text-muted-foreground">{label}</p>
  {/if}
  {@render children?.()}
</div>
