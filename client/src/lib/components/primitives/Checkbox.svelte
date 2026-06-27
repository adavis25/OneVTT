<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  interface Props extends Omit<HTMLButtonAttributes, 'role' | 'type' | 'aria-checked'> {
    checked?: boolean;
  }

  let { checked = $bindable(false), disabled = false, class: className, ...rest }: Props = $props();
</script>

<button
  type="button"
  role="checkbox"
  aria-checked={checked}
  {disabled}
  onclick={() => { if (!disabled) checked = !checked; }}
  class={cn(
    'inline-flex h-4 w-4 shrink-0 items-center justify-center rounded-sm border border-input bg-background transition-colors',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
    'disabled:cursor-not-allowed disabled:opacity-50',
    checked ? 'bg-primary border-primary' : 'cursor-pointer',
    className
  )}
  {...rest}
>
  {#if checked}
    <svg
      class="h-3 w-3 text-primary-foreground"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="3.5"
      stroke-linecap="round"
      stroke-linejoin="round"
      aria-hidden="true"
    >
      <polyline points="20 6 9 17 4 12"></polyline>
    </svg>
  {/if}
</button>
