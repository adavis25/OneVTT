<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  let { class: className, value = $bindable(), ...rest }: HTMLInputAttributes = $props();
</script>

<input
  bind:value
  class={cn(
    'flex h-9 w-full rounded-[var(--radius)] border border-input bg-background px-3 py-1 text-sm text-foreground shadow-sm',
    'placeholder:text-muted-foreground',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
    'disabled:cursor-not-allowed disabled:opacity-50',
    'transition-colors',
    className
  )}
  {...rest}
/>
