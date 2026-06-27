<script lang="ts">
  import { setContext } from 'svelte';
  import type { Snippet } from 'svelte';
  import { cn } from '$lib/utils';

  interface Props {
    value: string;
    class?: string;
    children?: Snippet;
  }

  let { value = $bindable(), class: className, children }: Props = $props();

  setContext('onevtt-tabs', {
    get active() { return value; },
    set(v: string) { value = v; }
  });
</script>

<div class={cn('', className)}>
  {@render children?.()}
</div>
