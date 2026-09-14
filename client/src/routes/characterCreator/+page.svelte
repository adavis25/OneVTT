<script lang="ts">
  import { onMount } from 'svelte';
  import CharacterCreator from './CharacterCreator.svelte';
  import { getConnection } from '$lib/state/connection.svelte';

  onMount(() => {
    getConnection().connect();
  });
</script>

<CharacterCreator />