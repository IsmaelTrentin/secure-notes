import { writable } from 'svelte/store';

export type App = {
	authed: boolean;
	loading: boolean;
};

function createApp() {
	const { subscribe, set, update } = writable<App>({ authed: false, loading: false });

	return {
		subscribe,
		set(newState: App) {
			set(newState);
		},
		update(newState: App) {
			update((old) => ({ ...old, ...newState }));
		}
	};
}

export const app = createApp();
