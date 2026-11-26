import { mount } from 'svelte';
import './app.css';
import App from './lib/App.svelte';

// Mount the shell. Boot (backend selection, first snapshot, recovery prompt)
// happens inside App.svelte so the UI state lives in one place.
const app = mount(App, { target: document.getElementById('app')! });

export default app;
