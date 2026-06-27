<script lang="ts">
  import Canvas from '$lib/renderer/Canvas.svelte';
  import Chat from '$lib/ui/components/Chat.svelte';
</script>

<div class="app">
  <Canvas />
  <div class="ui-layer">
    <Chat />
  </div>
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