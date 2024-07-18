import { invoke } from '@tauri-apps/api';
import { writable } from 'svelte/store';

export type AppConfig = {
	version: string;
	db_path: string;
};
const defaultConf: AppConfig = {
	version: '0.0.1',
	db_path: '<data>/db.sqlite'
};

function createConfig() {
	const { subscribe, set } = writable<AppConfig>(defaultConf);

	const reload = async () => {
		const data = await invoke<AppConfig>('reload_config');
		set(data);
	};

	invoke<AppConfig>('get_config')
		.then((config) => set(config))
		.catch((e) => console.error('failed to get config', e));

	return {
		subscribe,
		reload
	};
}

export const config = createConfig();
