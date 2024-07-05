/* eslint-disable @typescript-eslint/no-unused-vars */
import { goto } from '$app/navigation';
import { invoke } from '@tauri-apps/api';
import { cmdPal } from './stores/cmdpal';

type Handler = (e: KeyboardEvent) => void | Promise<void>;

const bindings = new Map<string, Handler>();

const handleKeydown = (e: KeyboardEvent) => {
  const shiftOn = e.shiftKey;
  const specialOn = e.ctrlKey || e.metaKey;
  const altOn = e.altKey;
  const key = !shiftOn ? e.key : e.key.toUpperCase();

  const cmd = `${specialOn ? '<s>' : ''}${altOn ? '<alt>' : ''}${key}`;
  console.debug('keybind:', cmd);

  const handler = bindings.get(cmd);
  handler && handler(e);
};

export function init() {
  document.addEventListener('keydown', handleKeydown);
  return () => {
    document.removeEventListener('keydown', handleKeydown);
  };
}
export function register(bind: string, handler: Handler) {
  bindings.set(bind, handler);
  return bind;
}
export function toString(bind: string) {
  // TODO: platform specific
  const special = '⌘';
  const str = bind
    .replace('<s>', special);
  return str.replace(/[A-Z]/g, `↑${str.charAt(1)}`)
    .toUpperCase();
}

export const openCmdPalette = register('<s>P', (e) => {
  e.preventDefault();
  cmdPal.update((ps) => ({
    ...ps,
    open: !ps.open || !(ps.element === 'cmdpal'),
    element: 'cmdpal'
  }));
});
export const openVaultFile = register('<s>p', (e) => {
  e.preventDefault();
  cmdPal.update((ps) => ({
    ...ps,
    open: !ps.open || !(ps.element === 'filepicker'),
    element: 'filepicker'
  }));
});
export const openVault = register('<s>O', e => {
  e.preventDefault();
  throw new Error('not implemented');
});
export const newFile = register('<s>n', async (e) => {
  e.preventDefault();
  await goto('/editor');
});
export const closeFIle = register('<s>w', (e) => {
  e.preventDefault();
})
export const logout = register('<s>l', async (e) => {
  e.preventDefault();
  await invoke<never>('logout');
});

const keybindsManager = { register, init };
export default keybindsManager;
