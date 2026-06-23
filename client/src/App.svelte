<script lang="ts">
  import './app.css';
  import { onMount } from 'svelte';
  import { getConnection } from './state/connection.svelte';
  import Canvas from './canvas/Canvas.svelte';
  import Chat from './ui/Chat.svelte';
  import Welcome from './ui/Welcome.svelte';

  onMount(() => {
    getConnection().connect();
  });
</script>

<main>
  <Welcome />
</main>

<style>
  main {
    display: flex;
    height: 100vh;
  }
</style>