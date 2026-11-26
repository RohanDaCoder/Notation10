import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// The Tauri shell loads the built bundle with `withGlobalTauri`; the UI talks
// to it through `window.__TAURI__.core.invoke` (see lib/ipc.ts), so no Tauri
// packages are needed here and the same bundle also runs in a plain browser
// against the mock backend.
export default defineConfig({
  plugins: [svelte()],
  base: './',
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    target: 'es2022',
  },
});
