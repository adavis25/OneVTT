<script lang="ts">
  import { getContext } from 'svelte';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    value: string;
    disabled?: boolean;
    class?: string;
    children?: Snippet;
  }

  let { value, disabled = false, class: className, children }: Props = $props();

  const tabs = getContext<{ active: string; set: (v: string) => void }>('onevtt-tabs');
  const isActive = $derived(tabs.active === value);
</script>

<button
  type="button"
  role="tab"
  aria-selected={isActive}
  {disabled}
  onclick={() => { if (!disabled) tabs.set(value); }}
  class={cn(
    'inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium transition-all',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
    'disabled:pointer-events-none disabled:opacity-50',
    isActive
      ? 'bg-background text-foreground shadow'
      : 'hover:bg-background/50 hover:text-foreground',
    className
  )}
>
  {@render children?.()}
</button>
