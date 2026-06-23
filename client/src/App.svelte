<script lang="ts">
  import { onMount } from 'svelte';
  import { getConnection } from './state/connection.svelte';
  import Canvas from './canvas/Canvas.svelte';
  import Chat from './ui/Chat.svelte';

  onMount(() => {
    getConnection().connect();
  });
</script>

<main>
  <Canvas />
  <Chat />
</main>

<style>
  main {
    display: flex;
    height: 100vh;
  }
</style>