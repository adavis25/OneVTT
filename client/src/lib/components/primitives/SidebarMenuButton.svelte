<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props extends Omit<HTMLButtonAttributes, 'type'> {
    name: string;
    subtitle?: string;
    icon?: Snippet;
  }

  let { name, subtitle, icon, class: className, ...rest }: Props = $props();
</script>

<button
  type="button"
  class={cn(
    'flex w-full items-center gap-3 rounded-[var(--radius)] px-2 py-2 text-left transition-colors',
    'hover:bg-accent hover:text-accent-foreground',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
    className
  )}
  {...rest}
>
  {#if icon}
    <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-[var(--radius)] bg-primary text-primary-foreground">
      {@render icon()}
    </span>
  {/if}

  <div class="flex-1 truncate">
    <div class="truncate text-sm font-semibold">{name}</div>
    {#if subtitle}
      <div class="truncate text-xs text-muted-foreground">{subtitle}</div>
    {/if}
  </div>

  <!-- Sort chevron -->
  <svg class="ml-auto h-4 w-4 shrink-0 text-muted-foreground" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <polyline points="8 15 12 19 16 15"></polyline>
    <polyline points="16 9 12 5 8 9"></polyline>
  </svg>
</button>
