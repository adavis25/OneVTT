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
  role="switch"
  aria-checked={checked}
  {disabled}
  onclick={() => { if (!disabled) checked = !checked; }}
  class={cn(
    'inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
    'disabled:cursor-not-allowed disabled:opacity-50',
    checked ? 'bg-primary' : 'bg-input',
    className
  )}
  {...rest}
>
  <span
    class={cn(
      'pointer-events-none block h-4 w-4 rounded-full bg-background shadow-lg transition-transform',
      checked ? 'translate-x-4' : 'translate-x-0'
    )}
  ></span>
</button>
