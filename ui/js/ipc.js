// IPC bridge to the Tauri backend (window.__TAURI__ is injected because the
// app config sets `withGlobalTauri`). When absent (plain browser), the mock
// backend in mock.js takes over so the UI can be developed and verified
// without a build step.

export const inTauri = () => typeof window !== 'undefined' && !!window.__TAURI__;

export async function invoke(cmd, args = {}) {
  return window.__TAURI__.core.invoke(cmd, args);
}

export async function listen(event, handler) {
  const l = window.__TAURI__?.event?.listen;
  if (typeof l !== 'function') return () => {};
  return l(event, handler);
}
