<script lang="ts">
  import { onMount } from 'svelte';
  import Sidebar from '$lib/components/primitives/Sidebar.svelte';
  import DropdownItem from '$lib/components/primitives/DropdownItem.svelte';
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

  interface ContextMenu {
    x: number;
    y: number;
    actor: Actor;
  }

  let activeTab = $state<Tab>('actors');
  let actors = $state<Actor[]>([]);
  let loadingActors = $state(false);
  let creatingActor = $state(false);
  let contextMenu = $state<ContextMenu | null>(null);

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
    } finally {
      creatingActor = false;
    }
  }

  async function removeActor(id: string) {
    contextMenu = null;
    await fetch(`/api/actors/${id}`, { method: 'DELETE' });
    // list updated via actor.removed broadcast below
  }

  function openContextMenu(e: MouseEvent, actor: Actor) {
    e.preventDefault();
    contextMenu = { x: e.clientX, y: e.clientY, actor };
  }

  function closeContextMenu() {
    contextMenu = null;
  }

  // Process only new messages; track index to avoid reprocessing old ones
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
        } else if (event.type === 'actor.removed') {
          actors = actors.filter(a => a.id !== event.data.id);
        }
      } catch { /* not a JSON game event */ }
    }
    lastMsgCount = msgs.length;
  });
</script>

<svelte:window
  onclick={closeContextMenu}
  onkeydown={(e) => { if (e.key === 'Escape') closeContextMenu(); }}
/>

<!-- Context menu -->
{#if contextMenu}
  <div
    class="fixed z-50 min-w-[8rem] overflow-hidden rounded-[var(--radius)] border border-border bg-card p-1 shadow-md"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px"
    role="menu"
    tabindex="-1"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <DropdownItem disabled>Edit</DropdownItem>
    <DropdownItem variant="destructive" onclick={() => removeActor(contextMenu!.actor.id)}>
      Remove
    </DropdownItem>
  </div>
{/if}

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
            <li
              class="flex items-center gap-2 px-4 py-2 text-sm hover:bg-accent hover:text-accent-foreground transition-colors cursor-pointer select-none"
              oncontextmenu={(e) => openContextMenu(e, actor)}
            >
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
