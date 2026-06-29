export interface GameSystem {
  id: string;
  label: string;
  ui: Record<string, unknown>;
  dice: Record<string, unknown>;
}

let activeSystem = $state<GameSystem | null>(null);
let locked = false;

export function defineSystem(system: GameSystem): void {
  if (locked) {
    console.warn(`[registry] defineSystem called with '${system.id}' but a system is already registered ('${activeSystem?.id}'). Ignoring.`);
    return;
  }
  activeSystem = system;
  locked = true;
}

export function getSystem() {
  return {
    get active() { return activeSystem; }
  };
}
