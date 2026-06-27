import { browser } from '$app/environment';

export type Theme = 'light' | 'dark' | 'darkula';
const STORAGE_KEY = 'onevtt-theme';
const THEME_CLASSES: Theme[] = ['dark', 'darkula'];

function readStored(): Theme {
  if (!browser) return 'dark';
  return (localStorage.getItem(STORAGE_KEY) as Theme) ?? 'dark';
}

let current = $state<Theme>(readStored());

export function getTheme() {
  return {
    get current() {
      return current;
    },
    set(t: Theme) {
      current = t;
      if (browser) {
        localStorage.setItem(STORAGE_KEY, t);
        THEME_CLASSES.forEach(c => document.documentElement.classList.remove(c));
        if (t !== 'light') document.documentElement.classList.add(t);
      }
    },
    init() {
      if (browser) {
        THEME_CLASSES.forEach(c => document.documentElement.classList.remove(c));
        if (current !== 'light') document.documentElement.classList.add(current);
      }
    }
  };
}
