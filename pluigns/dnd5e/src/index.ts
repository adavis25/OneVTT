import { defineSystem } from '$lib/state/registry.svelte';
import CharacterSheet from './ui/CharacterSheet.svelte';

defineSystem({
  id: 'dnd5e',
  label: 'Dungeons & Dragons 5th Edition',
  ui: {
    actorSheets: {
      character: CharacterSheet,
    },
    actorCreators: {},
  },
});
