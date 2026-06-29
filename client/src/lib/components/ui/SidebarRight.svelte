<script lang="ts">
  import Sidebar from '$lib/components/primitives/Sidebar.svelte';
  import { cn } from '$lib/utils';

  type Tab = 'actors' | 'items' | 'journal';

  const tabs: { id: Tab; label: string }[] = [
    { id: 'actors', label: 'Actors' },
    { id: 'items', label: 'Items' },
    { id: 'journal', label: 'Journal' },
  ];

  let activeTab = $state<Tab>('actors');
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

  <div role="tabpanel" class="p-4">
    {#if activeTab === 'actors'}
      <p class="text-sm text-muted-foreground">No actors yet.</p>
    {:else if activeTab === 'items'}
      <p class="text-sm text-muted-foreground">No items yet.</p>
    {:else if activeTab === 'journal'}
      <p class="text-sm text-muted-foreground">No journal entries yet.</p>
    {/if}
  </div>
</Sidebar>
