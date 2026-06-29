<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import Button from '$lib/components/primitives/Button.svelte';
  import Input from '$lib/components/primitives/Input.svelte';
  import Label from '$lib/components/primitives/Label.svelte';
  import Select from '$lib/components/primitives/Select.svelte';
  import { getWorld } from '$lib/state/world.svelte';

  type Role = 'gm' | 'player';

  interface WorldSummary {
    id: string;
    name: string;
    game_system: string;
    last_opened: number | null;
  }

  let role = $state<Role>('gm');
  let worlds = $state<WorldSummary[]>([]);
  let loadingWorlds = $state(false);
  let selectedWorldId = $state<string | null>(null);

  let newName = $state('');
  let newGameSystem = $state('dnd5e');
  let creating = $state(false);

  let serverAddress = $state('');

  const world = getWorld();

  onMount(fetchWorlds);

  async function fetchWorlds() {
    loadingWorlds = true;
    try {
      const res = await fetch('/api/worlds');
      if (res.ok) worlds = await res.json();
    } finally {
      loadingWorlds = false;
    }
  }

  async function createWorld() {
    if (!newName.trim() || creating) return;
    creating = true;
    try {
      const res = await fetch('/api/worlds', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: newName.trim(), game_system: newGameSystem })
      });
      if (res.ok) {
        const created: { id: string; name: string; game_system: string } = await res.json();
        world.set({ id: created.id, name: created.name, game_system: created.game_system, last_opened: null });
        goto('/game');
      }
    } finally {
      creating = false;
    }
  }

  function enterWorld() {
    const w = worlds.find(w => w.id === selectedWorldId);
    if (!w) return;
    world.set(w);
    goto('/game');
  }

  async function removeWorld(id: string) {
    const res = await fetch(`/api/worlds/${id}`, { method: 'DELETE' });
    if (res.ok) {
      worlds = worlds.filter(w => w.id !== id);
      if (selectedWorldId === id) selectedWorldId = null;
    }
  }
</script>

<div class="w-full min-h-screen bg-background text-foreground flex items-center justify-center">
  <div class="w-full max-w-md flex flex-col gap-6 p-8">

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

    {#if role === 'gm'}
      <!-- Existing worlds -->
      {#if loadingWorlds}
        <p class="text-muted-foreground text-sm text-center">Loading worlds...</p>
      {:else if worlds.length > 0}
        <div class="flex flex-col gap-2">
          <p class="text-sm font-medium">Your worlds</p>
          {#each worlds as w (w.id)}
            <div
              class="flex items-center justify-between p-3 rounded-[var(--radius)] border cursor-pointer transition-colors
                {selectedWorldId === w.id
                  ? 'border-primary bg-primary/10 text-foreground'
                  : 'border-border hover:border-primary/50 text-foreground'}"
              onclick={() => selectedWorldId = w.id}
              role="button"
              tabindex="0"
              onkeydown={(e) => e.key === 'Enter' && (selectedWorldId = w.id)}
            >
              <div>
                <p class="text-sm font-medium">{w.name}</p>
                <p class="text-xs text-muted-foreground">{w.game_system}</p>
              </div>
              <button
                class="ml-3 text-muted-foreground hover:text-destructive transition-colors"
                title="Remove world"
                onclick={(e) => { e.stopPropagation(); removeWorld(w.id); }}
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                  <polyline points="3 6 5 6 21 6"></polyline>
                  <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"></path>
                  <path d="M10 11v6"></path>
                  <path d="M14 11v6"></path>
                  <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2"></path>
                </svg>
              </button>
            </div>
          {/each}

          {#if selectedWorldId}
            <Button size="lg" class="w-full mt-1" onclick={enterWorld}>Enter World</Button>
          {/if}
        </div>

        <!-- Divider -->
        <div class="flex items-center gap-3">
          <div class="flex-1 border-t border-border"></div>
          <span class="text-xs text-muted-foreground">or create new</span>
          <div class="flex-1 border-t border-border"></div>
        </div>
      {/if}

      <!-- Create world form -->
      <div class="flex flex-col gap-3">
        {#if worlds.length === 0 && !loadingWorlds}
          <p class="text-sm text-muted-foreground text-center">No worlds yet — create your first one.</p>
        {/if}
        <div class="flex flex-col gap-1.5">
          <Label for="world-name">World name</Label>
          <Input id="world-name" type="text" bind:value={newName} placeholder="My Campaign" />
        </div>
        <div class="flex flex-col gap-1.5">
          <Label for="game-system">Game system</Label>
          <Select id="game-system" bind:value={newGameSystem}>
            <option value="dnd5e">Dungeons &amp; Dragons 5th Edition</option>
          </Select>
        </div>
        <Button
          size="lg"
          class="w-full"
          onclick={createWorld}
          disabled={!newName.trim() || creating}
        >
          {creating ? 'Creating...' : 'Create & Enter'}
        </Button>
      </div>

    {:else}
      <!-- Player view -->
      <div class="flex flex-col gap-1.5">
        <Label for="server-address">Server address</Label>
        <Input id="server-address" type="text" bind:value={serverAddress} placeholder="192.168.x.x:3000" />
      </div>
      <Button size="lg" class="w-full" disabled={!serverAddress.trim()} onclick={() => goto('/game')}>
        Connect
      </Button>
    {/if}

  </div>
</div>
