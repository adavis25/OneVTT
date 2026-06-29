<script lang="ts">
  import Canvas from '$lib/components/canvas/Canvas.svelte';
  import SidebarLeft from '$lib/components/ui/SidebarLeft.svelte';
  import SidebarRight from '$lib/components/ui/SidebarRight.svelte';
</script>

<div class="app">
  <Canvas />
  <div class="ui-layer">
    <SidebarLeft />
    <SidebarRight />
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