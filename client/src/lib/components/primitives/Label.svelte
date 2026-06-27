<script lang="ts">
  import type { HTMLLabelAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props extends HTMLLabelAttributes {
    children?: Snippet;
  }

  let { class: className, children, ...rest }: Props = $props();
</script>

<label
  class={cn(
    'text-sm font-medium leading-none text-foreground',
    'peer-disabled:cursor-not-allowed peer-disabled:opacity-70',
    className
  )}
  {...rest}
>
  {@render children?.()}
</label>
