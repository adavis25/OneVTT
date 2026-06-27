<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props extends Omit<HTMLButtonAttributes, 'role'> {
    variant?: 'default' | 'destructive';
    children?: Snippet;
  }

  let { variant = 'default', class: className, children, ...rest }: Props = $props();
</script>

<button
  type="button"
  role="menuitem"
  class={cn(
    'flex w-full cursor-pointer items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-none transition-colors',
    'focus:bg-accent focus:text-accent-foreground',
    'hover:bg-accent hover:text-accent-foreground',
    'disabled:pointer-events-none disabled:opacity-50',
    variant === 'destructive' && 'text-destructive hover:bg-destructive/10 focus:bg-destructive/10',
    className
  )}
  {...rest}
>
  {@render children?.()}
</button>
