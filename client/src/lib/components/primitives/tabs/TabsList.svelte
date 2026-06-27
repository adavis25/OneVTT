<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    class?: string;
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();
</script>

<div
  role="tablist"
  class={cn(
    'inline-flex items-center justify-center rounded-[var(--radius)] bg-muted p-1 text-muted-foreground',
    className
  )}
>
  {@render children?.()}
</div>
