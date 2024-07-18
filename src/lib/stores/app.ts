import { writable } from 'svelte/store';
import type { Vault } from './vault.picker';

export type App = {
	authed: boolean;
	loading: boolean;
	vault?: Vault;
};

function createApp() {
	const { subscribe, set, update } = writable<App>({ authed: false, loading: false });

	return {
		subscribe,
		update,
		set(newState: App) {
			set(newState);
		}
	};
}

export const app = createApp();
