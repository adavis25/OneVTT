import type { Component } from 'svelte';

export interface SidebarPanel {
  id: string;
  label: string;
  component: Component;
}

export interface GameSystem {
  id: string;
  label: string;
  ui: {
    actorSheets: Record<string, Component>;
    actorCreators: Record<string, Component>;
    sidebarPanels?: SidebarPanel[];
    diceTray?: Component;
  };
  dice?: unknown;
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
