<script lang="ts">
  import Canvas from '$lib/renderer/Canvas.svelte';
  import Chat from '$lib/ui/components/Chat.svelte';
  import * as Sidebar from '$lib/components/ui/sidebar/index.js';
  import AppSidebar from '$lib/ui/components/app-sidebar.svelte';
</script>

<div class="app">
  <Canvas />
  <div class="ui-layer">
    <Sidebar.Provider>
      <AppSidebar />
      <main class="flex flex-col flex-1 bg-transparent">
        <div class="flex items-center gap-2 p-2">
          <Sidebar.Trigger />
        </div>
        <Chat />
      </main>
    </Sidebar.Provider>
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