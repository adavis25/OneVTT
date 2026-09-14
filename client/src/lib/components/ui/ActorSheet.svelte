<script lang="ts">
  import type { Component } from 'svelte';
  import { getSystem } from '$lib/state/registry.svelte';

  export interface ActorFull {
    id: string;
    name: string;
    actor_type: string;
    data: Record<string, unknown>;
    world_id?: string;
  }

  interface Props {
    actor: ActorFull;
    onClose: () => void;
  }

  let { actor, onClose }: Props = $props();

  const system = getSystem();

  // Capital name so Svelte treats it as a dynamic component in the template
  type ActorSheetComp = Component<{ actor: ActorFull }>;
  let SheetComponent = $derived<ActorSheetComp | null>(
    (system.active?.ui.actorSheets[actor.actor_type] as ActorSheetComp) ?? null
  );
</script>

<!-- Backdrop -->
<div
  class="fixed inset-0 z-40 bg-background/80 backdrop-blur-sm flex items-center justify-center p-4"
  role="presentation"
  onclick={onClose}
  onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
>
  <!-- Panel — stops backdrop clicks -->
  <div
    class="relative z-50 w-full max-w-2xl max-h-[80vh] flex flex-col rounded-[var(--radius)] border border-border bg-card shadow-xl"
    role="dialog"
    aria-modal="true"
    aria-label={actor.name}
    tabindex="-1"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between shrink-0 border-b border-border px-6 py-4">
      <div>
        <h2 class="text-lg font-semibold text-foreground">{actor.name}</h2>
        <p class="text-xs text-muted-foreground">{actor.actor_type}</p>
      </div>
      <button
        class="text-muted-foreground hover:text-foreground transition-colors"
        aria-label="Close"
        onclick={onClose}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
    </div>

    <!-- Sheet body -->
    <div class="flex-1 overflow-y-auto p-6">
      {#if SheetComponent}
        <SheetComponent {actor} />
      {:else}
        <p class="text-sm text-muted-foreground">
          No sheet registered for actor type <span class="font-mono text-foreground">"{actor.actor_type}"</span>.
        </p>
      {/if}
    </div>
  </div>
</div>
