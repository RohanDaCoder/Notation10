// IPC bridge to the Tauri backend. The app config sets `withGlobalTauri`, so
// the runtime injects `window.__TAURI__`. When absent (plain browser), the
// mock backend in mock.ts takes over through the same surface, keeping the
// UI runnable without a native build.

export type Unlisten = () => void;

export function inTauri(): boolean {
  return typeof window !== 'undefined' && !!(window as any).__TAURI__;
}

export async function invoke<T = any>(cmd: string, args: Record<string, unknown> = {}): Promise<T> {
  const tauri = (window as any).__TAURI__;
  return tauri.core.invoke(cmd, args) as Promise<T>;
}

export async function listen(event: string, handler: (ev: any) => void): Promise<Unlisten> {
  const l = (window as any).__TAURI__?.event?.listen;
  if (typeof l !== 'function') return () => {};
  return l(event, handler) as Promise<Unlisten>;
}
