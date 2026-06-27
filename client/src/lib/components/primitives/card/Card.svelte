<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    class?: string;
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();
</script>

<div class={cn('rounded-[var(--radius)] border border-border bg-card text-card-foreground shadow-sm', className)}>
  {@render children?.()}
</div>
