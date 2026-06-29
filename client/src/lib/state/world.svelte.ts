import { browser } from '$app/environment';

export interface World {
  id: string;
  name: string;
  game_system: string;
  last_opened: number | null;
}

const STORAGE_KEY = 'onevtt-active-world';

function readStored(): World | null {
  if (!browser) return null;
  const raw = localStorage.getItem(STORAGE_KEY);
  if (!raw) return null;
  try {
    return JSON.parse(raw) as World;
  } catch {
    return null;
  }
}

let activeWorld = $state<World | null>(readStored());

export function getWorld() {
  return {
    get active() { return activeWorld; },
    set(world: World) {
      activeWorld = world;
      if (browser) localStorage.setItem(STORAGE_KEY, JSON.stringify(world));
    },
    clear() {
      activeWorld = null;
      if (browser) localStorage.removeItem(STORAGE_KEY);
    }
  };
}