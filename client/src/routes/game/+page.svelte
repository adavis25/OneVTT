<script lang="ts">
  import { onMount } from 'svelte';
  import Game from './Game.svelte';
  import { getConnection } from '$lib/state/connection.svelte';
  import { getWorld } from '$lib/state/world.svelte';

  const world = getWorld();

  onMount(() => {
    const activeWorld = world.active;
    console.log('[game] active world:', activeWorld?.name ?? '(none)');
    console.log('[game] game system:', activeWorld?.game_system ?? '(none)');
    getConnection().connect();
  });
</script>

<Game />