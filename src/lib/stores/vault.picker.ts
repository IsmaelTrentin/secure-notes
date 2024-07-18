import { writable } from 'svelte/store';

export type Vault = {
	name: string;
	path: string;
	icon?: string;
};
type VaultPicker = {
	vaults: Vault[];
	open: boolean;
	value: string | undefined;
};

function createVaultPicker() {
	const { subscribe, set, update } = writable<VaultPicker>({
		vaults: [],
		open: false,
		value: undefined
	});

	return { subscribe, set, update };
}

export const vaultPicker = createVaultPicker();
