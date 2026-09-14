<script lang="ts">
  import Sidebar from '$lib/components/primitives/Sidebar.svelte';
  import { getWorld } from '$lib/state/world.svelte';

  const world = getWorld();
</script>

<Sidebar contained side="left">
  {#snippet header()}
    <div class="flex flex-col gap-0.5 px-4 py-3">
      <p class="text-sm font-semibold text-foreground truncate">
        {world.active?.name ?? 'No world loaded'}
      </p>
      <p class="text-xs text-muted-foreground">
        {world.active?.game_system ?? ''}
      </p>
    </div>
  {/snippet}
</Sidebar>
