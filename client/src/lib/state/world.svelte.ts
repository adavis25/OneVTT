interface World {
  id: string;
  name: string;
  game_system: string;
  last_opened: number | null;
}

let activeWorld = $state<World | null>(null);

export function getWorld() {
  return {
    get active() { return activeWorld; },
    set(world: World) {
      activeWorld = world;
    },
    clear() {
      activeWorld = null;
    }
  };
}