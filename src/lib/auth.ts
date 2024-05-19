import { invoke } from '@tauri-apps/api';

export const auth = async () => {
	return await invoke<boolean>('is_authenticated');
};
