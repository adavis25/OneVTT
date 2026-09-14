<script lang="ts">
  import type { ActorFull } from '$lib/components/ui/ActorSheet.svelte';

  interface Props {
    actor: ActorFull;
  }

  let { actor }: Props = $props();

  const dataJson = $derived(JSON.stringify(actor.data, null, 2));
</script>

<div class="flex flex-col gap-4">
  <div class="grid grid-cols-2 gap-3 text-sm">
    <div>
      <p class="text-xs text-muted-foreground uppercase tracking-wide">Name</p>
      <p class="text-foreground font-medium">{actor.name}</p>
    </div>
    <div>
      <p class="text-xs text-muted-foreground uppercase tracking-wide">Type</p>
      <p class="text-foreground font-medium">{actor.actor_type}</p>
    </div>
  </div>

  <div>
    <p class="text-xs text-muted-foreground uppercase tracking-wide mb-1">Data</p>
    <pre class="rounded-[var(--radius)] border border-border bg-background p-3 text-xs text-foreground overflow-x-auto">{dataJson}</pre>
  </div>
</div>
