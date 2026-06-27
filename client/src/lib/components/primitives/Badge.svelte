<script lang="ts">
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  type Variant = 'default' | 'secondary' | 'outline' | 'destructive';

  interface Props {
    variant?: Variant;
    class?: string;
    children?: Snippet;
  }

  let { variant = 'default', class: className, children }: Props = $props();

  const variantClasses: Record<Variant, string> = {
    default:     'bg-primary text-primary-foreground',
    secondary:   'bg-secondary text-secondary-foreground',
    outline:     'border border-border text-foreground',
    destructive: 'bg-destructive text-destructive-foreground',
  };
</script>

<span
  class={cn(
    'inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold',
    variantClasses[variant],
    className
  )}
>
  {@render children?.()}
</span>
