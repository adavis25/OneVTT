<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props extends Omit<HTMLButtonAttributes, 'type'> {
    icon?: Snippet;
    arrow?: boolean;
    active?: boolean;
  }

  let { icon, arrow = false, active = false, class: className, children, ...rest }: Props = $props();
</script>

<button
  type="button"
  class={cn(
    'flex w-full items-center gap-3 rounded-[var(--radius)] px-2 py-2 text-sm transition-colors',
    'hover:bg-accent hover:text-accent-foreground',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
    'disabled:pointer-events-none disabled:opacity-50',
    active && 'bg-accent text-accent-foreground',
    className
  )}
  {...rest}
>
  {#if icon}
    <span class="flex h-5 w-5 shrink-0 items-center justify-center text-muted-foreground">
      {@render icon()}
    </span>
  {/if}

  <span class="flex-1 truncate text-left">{@render children?.()}</span>

  {#if arrow}
    <svg class="ml-auto h-4 w-4 shrink-0 text-muted-foreground" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
      <polyline points="9 18 15 12 9 6"></polyline>
    </svg>
  {/if}
</button>
