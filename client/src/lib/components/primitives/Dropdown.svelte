<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    align?: 'start' | 'end';
    class?: string;
    trigger: Snippet;
    children?: Snippet;
  }

  let { align = 'start', class: className, trigger, children }: Props = $props();

  let open = $state(false);

  function clickOutside(node: HTMLElement) {
    function handle(e: MouseEvent) {
      if (!node.contains(e.target as Node)) open = false;
    }
    function handleKey(e: KeyboardEvent) {
      if (e.key === 'Escape') open = false;
    }
    document.addEventListener('click', handle, true);
    document.addEventListener('keydown', handleKey);
    return {
      destroy() {
        document.removeEventListener('click', handle, true);
        document.removeEventListener('keydown', handleKey);
      }
    };
  }
</script>

<div class={cn('relative inline-block', className)} use:clickOutside>
  <div
    onclick={() => open = !open}
    onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') open = !open; }}
    role="presentation"
  >
    {@render trigger()}
  </div>

  {#if open}
    <div
      class={cn(
        'absolute top-full z-50 mt-1 min-w-[8rem] overflow-hidden rounded-[var(--radius)] border border-border bg-card p-1 shadow-md',
        align === 'end' ? 'right-0' : 'left-0'
      )}
      role="menu"
    >
      {@render children?.()}
    </div>
  {/if}
</div>
