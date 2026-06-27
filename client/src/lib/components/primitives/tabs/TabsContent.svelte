<script lang="ts">
  import { getContext } from 'svelte';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    value: string;
    class?: string;
    children?: Snippet;
  }

  let { value, class: className, children }: Props = $props();

  const tabs = getContext<{ active: string; set: (v: string) => void }>('onevtt-tabs');
  const isActive = $derived(tabs.active === value);
</script>

{#if isActive}
  <div
    role="tabpanel"
    class={cn('mt-4 focus-visible:outline-none', className)}
  >
    {@render children?.()}
  </div>
{/if}
