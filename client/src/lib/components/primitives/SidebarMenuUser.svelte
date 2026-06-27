<script lang="ts">
  import type { HTMLButtonAttributes } from 'svelte/elements';
  import { cn } from '$lib/utils';

  interface Props extends Omit<HTMLButtonAttributes, 'type'> {
    name: string;
    email?: string;
    avatar?: string;
  }

  let { name, email, avatar, class: className, ...rest }: Props = $props();

  const initials = $derived(
    name.split(' ').map(w => w[0]).slice(0, 2).join('').toUpperCase()
  );
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
  {#if avatar}
    <img src={avatar} alt={name} class="h-8 w-8 shrink-0 rounded-full object-cover" />
  {:else}
    <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-primary text-xs font-semibold text-primary-foreground">
      {initials}
    </span>
  {/if}

  <div class="flex-1 truncate">
    <div class="truncate text-sm font-semibold">{name}</div>
    {#if email}
      <div class="truncate text-xs text-muted-foreground">{email}</div>
    {/if}
  </div>

  <!-- Sort chevron -->
  <svg class="ml-auto h-4 w-4 shrink-0 text-muted-foreground" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <polyline points="8 15 12 19 16 15"></polyline>
    <polyline points="16 9 12 5 8 9"></polyline>
  </svg>
</button>
