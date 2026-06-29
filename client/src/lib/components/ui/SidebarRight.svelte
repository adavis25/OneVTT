<script lang="ts">
  import { onMount } from 'svelte';
  import Sidebar from '$lib/components/primitives/Sidebar.svelte';
  import { cn } from '$lib/utils';
  import { getWorld } from '$lib/state/world.svelte';
  import { getConnection } from '$lib/state/connection.svelte';

  type Tab = 'actors' | 'items' | 'journal';

  const tabs: { id: Tab; label: string }[] = [
    { id: 'actors', label: 'Actors' },
    { id: 'items', label: 'Items' },
    { id: 'journal', label: 'Journal' },
  ];

  interface Actor {
    id: string;
    name: string;
    actor_type: string;
  }

  let activeTab = $state<Tab>('actors');
  let actors = $state<Actor[]>([]);
  let loadingActors = $state(false);
  let creatingActor = $state(false);

  const world = getWorld();
  const conn = getConnection();

  onMount(fetchActors);

  async function fetchActors() {
    const worldId = world.active?.id;
    if (!worldId) return;
    loadingActors = true;
    try {
      const res = await fetch(`/api/actors?world_id=${encodeURIComponent(worldId)}`);
      if (res.ok) actors = await res.json();
    } finally {
      loadingActors = false;
    }
  }

  async function createActor() {
    const worldId = world.active?.id;
    if (!worldId || creatingActor) return;
    creatingActor = true;
    try {
      await fetch('/api/actors', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          world_id: worldId,
          name: 'New Actor',
          actor_type: 'character',
          data: {}
        })
      });
      // list updated via actor.created broadcast below
    } finally {
      creatingActor = false;
    }
  }

  // Process only new messages on each change to avoid reprocessing old ones
  let lastMsgCount = 0;

  $effect(() => {
    const msgs = conn.messages;
    for (let i = lastMsgCount; i < msgs.length; i++) {
      try {
        const event = JSON.parse(msgs[i]);
        if (event.type === 'actor.created') {
          const a = event.data as Actor;
          if (!actors.some(x => x.id === a.id)) {
            actors = [...actors, a];
          }
        }
      } catch { /* not a JSON game event */ }
    }
    lastMsgCount = msgs.length;
  });
</script>

<Sidebar contained side="right">
  {#snippet header()}
    <div class="flex" role="tablist">
      {#each tabs as tab (tab.id)}
        <button
          role="tab"
          aria-selected={activeTab === tab.id}
          class={cn(
            'flex-1 py-2.5 text-xs font-medium transition-colors border-b-2',
            activeTab === tab.id
              ? 'border-primary text-foreground'
              : 'border-transparent text-muted-foreground hover:text-foreground'
          )}
          onclick={() => activeTab = tab.id}
        >{tab.label}</button>
      {/each}
    </div>
  {/snippet}

  <div role="tabpanel" class="flex flex-col h-full">
    {#if activeTab === 'actors'}
      <div class="flex items-center justify-between px-4 py-2 border-b border-border">
        <span class="text-xs font-medium text-muted-foreground uppercase tracking-wide">Actors</span>
        <button
          class="text-muted-foreground hover:text-foreground transition-colors disabled:opacity-50"
          title="New actor"
          disabled={creatingActor}
          onclick={createActor}
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <line x1="12" y1="5" x2="12" y2="19"></line>
            <line x1="5" y1="12" x2="19" y2="12"></line>
          </svg>
        </button>
      </div>

      {#if loadingActors}
        <p class="px-4 py-3 text-sm text-muted-foreground">Loading...</p>
      {:else if actors.length === 0}
        <p class="px-4 py-3 text-sm text-muted-foreground">No actors yet.</p>
      {:else}
        <ul class="flex flex-col">
          {#each actors as actor (actor.id)}
            <li class="flex items-center gap-2 px-4 py-2 text-sm hover:bg-accent hover:text-accent-foreground transition-colors cursor-pointer">
              <span class="flex-1 truncate">{actor.name}</span>
              <span class="text-xs text-muted-foreground shrink-0">{actor.actor_type}</span>
            </li>
          {/each}
        </ul>
      {/if}

    {:else if activeTab === 'items'}
      <p class="px-4 py-3 text-sm text-muted-foreground">No items yet.</p>

    {:else if activeTab === 'journal'}
      <p class="px-4 py-3 text-sm text-muted-foreground">No journal entries yet.</p>
    {/if}
  </div>
</Sidebar>
