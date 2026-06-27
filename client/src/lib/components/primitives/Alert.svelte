<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  type Variant = 'default' | 'destructive';

  interface Props {
    variant?: Variant;
    title?: string;
    class?: string;
    icon?: Snippet;
    children?: Snippet;
  }

  let { variant = 'default', title, class: className, icon, children }: Props = $props();

  const variantClasses: Record<Variant, string> = {
    default:     'border-border bg-card text-card-foreground',
    destructive: 'border-destructive/50 bg-destructive/10 text-destructive',
  };
</script>

<div
  role="alert"
  class={cn(
    'relative w-full rounded-[var(--radius)] border p-4',
    variantClasses[variant],
    className
  )}
>
  {#if icon || title}
    <div class="mb-1 flex items-center gap-2">
      {#if icon}
        <span class="shrink-0">{@render icon()}</span>
      {/if}
      {#if title}
        <h5 class="font-medium leading-none tracking-tight">{title}</h5>
      {/if}
    </div>
  {/if}
  {#if children}
    <div class="text-sm leading-relaxed opacity-90">{@render children()}</div>
  {/if}
</div>
