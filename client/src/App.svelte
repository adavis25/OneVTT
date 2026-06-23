<script lang="ts">
  import './app.css';
  import { getPath, navigate } from './state/router.svelte';

  import Canvas from './canvas/Canvas.svelte';
  import Welcome from './ui/Welcome.svelte';

  const path = $derived(getPath());
  
</script>

<div>
  {#if path === '/'}
    <Welcome onEnter={() => navigate('/game')} />
  {:else if path === '/game'}
    <Canvas />
  {:else if path === '/settings'}
    <!-- Settings placeholder -->
  {/if}
</div>

<style>
  .app {
    position: relative;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }

  .ui-layer {
    position: absolute;
    inset: 0;
    pointer-events: none;
  }

  .ui-layer :global(*) {
    pointer-events: auto;
  }
</style>