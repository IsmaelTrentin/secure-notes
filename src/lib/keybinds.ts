/* eslint-disable @typescript-eslint/no-unused-vars */
import { goto } from '$app/navigation';
import { invoke } from '@tauri-apps/api';
import { cmdPal } from './stores/cmdpal';
import { vaultPicker } from './stores/vault.picker';
import { config, type AppConfig } from './stores/config';
import { toast } from 'svelte-sonner';
import { app } from './stores/app';
import { get } from 'svelte/store';

type Handler = (e: KeyboardEvent) => void | Promise<void>;

let appConfig: AppConfig;
config.subscribe((value) => {
	appConfig = value;
});

const bindings = new Map<string, Handler>();

const handleKeydown = (e: KeyboardEvent) => {
	if (!get(app).authed) return;

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
	const str = bind.replace('<s>', special);
	return str.replace(/[A-Z]/g, `↑${str.charAt(1)}`).toUpperCase();
}

export const openCmdPalette = register('<s>P', (e) => {
	e.preventDefault();
	cmdPal.update((ps) => ({
		...ps,
		open: !ps.open || !(ps.element === 'cmdpal'),
		element: 'cmdpal'
	}));
	vaultPicker.update((ps) => ({
		...ps,
		open: false
	}));
});
export const openVaultFile = register('<s>p', (e) => {
	e.preventDefault();
	cmdPal.update((ps) => ({
		...ps,
		open: !ps.open || !(ps.element === 'filepicker'),
		element: 'filepicker'
	}));
	vaultPicker.update((ps) => ({
		...ps,
		open: false
	}));
});
export const openVault = register('<s>O', (e) => {
	e.preventDefault();
	cmdPal.update((ps) => ({
		...ps,
		open: false
	}));
	vaultPicker.update((ps) => ({
		...ps,
		open: !ps.open
	}));
});
export const newFile = register('<s>n', async (e) => {
	e.preventDefault();
	await goto('/editor');
});
export const closeFIle = register('<s>w', (e) => {
	e.preventDefault();
});
export const logout = register('<s>l', async (e) => {
	e.preventDefault();
	cmdPal.update((ps) => ({ ...ps, open: false }));
	vaultPicker.update((ps) => ({ ...ps, open: false }));
	await invoke<never>('logout');
	app.update((ps) => ({ ...ps, authed: false }));
});

// DEBUG
export const reloadConfig = register('<alt>R', async (e) => {
	e.preventDefault();

	try {
		await config.reload();
		toast.success('Config reloaded', {
			duration: 650
		});
	} catch (error) {
		toast.error('Failed to reload config', {
			description: error as unknown as string
		});
	}
});

const keybindsManager = { register, init };
export default keybindsManager;
