<script lang="ts">
  import './app.css';
  import Canvas from './canvas/Canvas.svelte';
  import Chat from './ui/Chat.svelte';
  import Welcome from './ui/Welcome.svelte';

  let inGame = $state(false);

  function onEnter() {
    inGame = true;
  }
</script>

{#if inGame}
  <div class="app">
    <Canvas />
    <div class="ui-layer">
      <Chat />
    </div>
  </div>
{:else}
  <Welcome onEnter={() => onEnter()} />
{/if}

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