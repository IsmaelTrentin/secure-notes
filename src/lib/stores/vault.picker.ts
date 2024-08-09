import { writable } from 'svelte/store';

export type VaultEntry = { filename: string; path: string; children: VaultEntry[] };
export type Vault = {
	name: string;
	path: string;
	entries: VaultEntry[];
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
