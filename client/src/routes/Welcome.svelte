<script lang="ts">
  import { goto } from '$app/navigation';
  import Button from '$lib/components/primitives/Button.svelte';
  import Input from '$lib/components/primitives/Input.svelte';
  import Label from '$lib/components/primitives/Label.svelte';

  type Role = 'gm' | 'player';
  let role = $state<Role>('gm');
  let worldName = $state('');

  function handleEnter() {
    goto('/game');
  }
</script>

<div class="w-full min-h-screen bg-background text-foreground flex items-center justify-center">
  <div class="w-full max-w-md flex flex-col gap-8 p-8">

    <!-- Title -->
    <div class="text-center">
      <h1 class="text-4xl font-bold tracking-wide">OneVTT</h1>
      <p class="text-muted-foreground mt-2 text-sm">Virtual Tabletop</p>
    </div>

    <!-- Role toggle -->
    <div class="flex rounded-[var(--radius)] overflow-hidden border border-border">
      <button
        class="flex-1 py-2 text-sm font-medium transition-colors
          {role === 'gm'
            ? 'bg-secondary text-secondary-foreground'
            : 'bg-transparent text-muted-foreground hover:text-foreground'}"
        onclick={() => role = 'gm'}
      >Game Master</button>
      <button
        class="flex-1 py-2 text-sm font-medium transition-colors
          {role === 'player'
            ? 'bg-secondary text-secondary-foreground'
            : 'bg-transparent text-muted-foreground hover:text-foreground'}"
        onclick={() => role = 'player'}
      >Player</button>
    </div>

    <!-- Fields -->
    {#if role === 'gm'}
      <div class="flex flex-col gap-1.5">
        <Label for="world-name">World name</Label>
        <Input id="world-name" type="text" bind:value={worldName} placeholder="My Campaign" />
      </div>
    {:else}
      <div class="flex flex-col gap-1.5">
        <Label for="server-address">Server address</Label>
        <Input id="server-address" type="text" placeholder="192.168.x.x:3000" />
      </div>
    {/if}

    <!-- Enter -->
    <Button size="lg" class="w-full" onclick={handleEnter}>
      {role === 'gm' ? 'Launch World' : 'Connect'}
    </Button>

  </div>
</div>
