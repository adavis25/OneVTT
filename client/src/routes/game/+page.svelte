<script lang="ts">
  import { onMount } from 'svelte';
  import Game from './Game.svelte';
  import { getConnection } from '$lib/state/connection.svelte';

  onMount(() => {
    getConnection().connect();
  });
</script>

<Game />