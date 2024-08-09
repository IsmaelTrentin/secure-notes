import { writable } from 'svelte/store';
import type { Vault } from './vault.picker';

export type Buffer = {
	id: string;
	filename: string;
	path: string;
};

export type App = {
	authed: boolean;
	loading: boolean;
	vault?: Vault;
	openBuffers: Map<string, Buffer>;
	selectedBufferId: string | undefined;
};

function createApp() {
	const { subscribe, set, update } = writable<App>({
		authed: false,
		loading: false,
		openBuffers: new Map(),
		selectedBufferId: undefined
	});

	return {
		subscribe,
		update,
		set(newState: App) {
			set(newState);
		}
	};
}

export const app = createApp();
