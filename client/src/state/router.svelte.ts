// src/state/router.svelte.ts
let currentPath = $state(window.location.pathname);

window.addEventListener('popstate', () => {
  currentPath = window.location.pathname;
});

export function navigate(path: string) {
  window.history.pushState({}, '', path);
  currentPath = path;
}

export function getPath() {
  return currentPath;
}