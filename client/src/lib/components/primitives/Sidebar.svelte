<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    open?: boolean;
    side?: 'left' | 'right';
    width?: string;
    /** Use absolute instead of fixed — for demos or contained layouts */
    contained?: boolean;
    class?: string;
    header?: Snippet;
    footer?: Snippet;
    children?: Snippet;
  }

  let {
    open = $bindable(true),
    side = 'left',
    width = '260px',
    contained = false,
    class: className,
    header,
    footer,
    children
  }: Props = $props();
</script>

<aside
  class={cn(
    'top-0 bottom-0 z-30 flex flex-col bg-card transition-transform duration-200 ease-in-out',
    contained ? 'absolute' : 'fixed',
    side === 'left' ? 'left-0 border-r border-border' : 'right-0 border-l border-border',
    !open && (side === 'left' ? '-translate-x-full' : 'translate-x-full'),
    className
  )}
  style="width: {width}"
  aria-hidden={!open}
>
  {#if header}
    <div class="shrink-0 border-b border-border">
      {@render header()}
    </div>
  {/if}

  <div class="flex-1 overflow-y-auto">
    {@render children?.()}
  </div>

  {#if footer}
    <div class="shrink-0 border-t border-border">
      {@render footer()}
    </div>
  {/if}
</aside>
