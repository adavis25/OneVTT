<script lang="ts">
  import type { HTMLSelectAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props extends HTMLSelectAttributes {
    children?: Snippet;
  }

  let { class: className, value = $bindable(), children, ...rest }: Props = $props();
</script>

<!-- wrapper takes the class so callers can constrain width -->
<div class={cn('relative', className)}>
  <select
    bind:value
    class="flex h-9 w-full appearance-none rounded-[var(--radius)] border border-input bg-background px-3 py-1 pr-8 text-sm text-foreground shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
    {...rest}
  >
    {@render children?.()}
  </select>
  <svg
    class="pointer-events-none absolute right-2.5 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground"
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <polyline points="6 9 12 15 18 9"></polyline>
  </svg>
</div>
