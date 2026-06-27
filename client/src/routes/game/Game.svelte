<script lang="ts">
  import Canvas from '$lib/components/canvas/Canvas.svelte';
</script>

<div class="app">
  <Canvas />
  <div class="ui-layer">
    
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
    display: flex;
    pointer-events: none;
  }

  .ui-layer :global(*) {
    pointer-events: auto;
  }
</style>