<script lang="ts">
  type Role = 'gm' | 'player';

  let { onEnter } = $props();
  let role = $state<Role>('gm');
  let worldName = $state('');

  function handleEnter() {
    // stub — will wire up later
    console.log('Enter world:', { role, worldName });
    onEnter();
  }

  
</script>

<div class="w-full min-h-screen bg-gray-950 text-gray-100 flex items-center justify-center">
  <div class="w-full max-w-md flex flex-col gap-8 p-8">

    <!-- Title -->
    <div class="text-center">
      <h1 class="text-4xl font-bold tracking-wide">Project VTT</h1>
      <p class="text-gray-500 mt-2 text-sm">Virtual Tabletop</p>
    </div>

    <!-- Role toggle -->
    <div class="flex rounded-lg overflow-hidden border border-gray-800">
      <button
        class="flex-1 py-2 text-sm font-medium transition-colors
          {role === 'gm' ? 'bg-gray-700 text-white' : 'bg-transparent text-gray-500 hover:text-gray-300'}"
        onclick={() => role = 'gm'}
      >
        Game Master
      </button>
      <button
        class="flex-1 py-2 text-sm font-medium transition-colors
          {role === 'player' ? 'bg-gray-700 text-white' : 'bg-transparent text-gray-500 hover:text-gray-300'}"
        onclick={() => role = 'player'}
      >
        Player
      </button>
    </div>

    <!-- Fields -->
    {#if role === 'gm'}
      <div class="flex flex-col gap-2">
        <label class="text-sm text-gray-400" for="world-name">World name</label>
        <input
          id="world-name"
          type="text"
          bind:value={worldName}
          placeholder="My Campaign"
          class="bg-gray-900 border border-gray-700 rounded-lg px-4 py-2
                 text-gray-100 placeholder-gray-600 focus:outline-none
                 focus:border-gray-500 transition-colors"
        />
      </div>
    {:else}
      <div class="flex flex-col gap-2">
        <label class="text-sm text-gray-400" for="server-address">Server address</label>
        <input
          id="server-address"
          type="text"
          placeholder="192.168.x.x:3000"
          class="bg-gray-900 border border-gray-700 rounded-lg px-4 py-2
                 text-gray-100 placeholder-gray-600 focus:outline-none
                 focus:border-gray-500 transition-colors"
        />
      </div>
    {/if}

    <!-- Enter button -->
    <button
      onclick={handleEnter}
      class="w-full py-3 rounded-lg bg-gray-700 hover:bg-gray-600
             font-medium transition-colors"
    >
      {role === 'gm' ? 'Launch World' : 'Connect'}
    </button>

  </div>
</div>