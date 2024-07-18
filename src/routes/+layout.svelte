<script lang="ts">
	import '../app.pcss';
	import { ModeWatcher } from 'mode-watcher';
	import Login from '$lib/components/Login.svelte';
	import { listen } from '@tauri-apps/api/event';
	import { app } from '$lib/stores/app';
	import { invoke } from '@tauri-apps/api';
	import AuthSetup from '$lib/components/AuthSetup.svelte';
	import { onMount } from 'svelte';
	import { open } from '@tauri-apps/api/dialog';
	import keybindsManager, {
		logout,
		newFile,
		openVault,
		openVaultFile,
		toString
	} from '$lib/keybinds';
	import * as Command from '$lib/components/ui/command';
	import Settings from 'lucide-svelte/icons/settings';
	import Vault from 'lucide-svelte/icons/vault';
	import Plus from 'lucide-svelte/icons/plus';
	import type { Vault as VaultType } from '$lib/stores/vault.picker';
	import File from 'lucide-svelte/icons/file';
	import FilePen from 'lucide-svelte/icons/file-pen';
	import { cmdPal } from '$lib/stores/cmdpal';
	import { Toaster } from '$lib/components/ui/sonner';
	import { vaultPicker } from '$lib/stores/vault.picker';
	import { goto } from '$app/navigation';

	let cmdValue: string | undefined;
	// $: console.log('VALUE', cmdValue);

	invoke<VaultType[]>('get_vaults').then((vaults) => ($vaultPicker.vaults = [...vaults]));

	let needs_auth_setup = false;
	const fetchAuthState = async () => {
		$app.loading = true;
		$app.authed = await invoke<boolean>('is_authenticated');
		needs_auth_setup = await invoke<boolean>('needs_auth_setup');
		$app.loading = false;
	};

	fetchAuthState();
	listen('auth_ok', async () => {
		$app.authed = true;
	});
	listen('auth_setup_ok', async () => {
		needs_auth_setup = false;
	});
	listen('logout', () => {
		$app.authed = false;
	});

	onMount(() => {
		return keybindsManager.init();
	});

	const onOpenVault = (vault: VaultType) => {
		console.log(vault);
		$app.vault = vault;
		localStorage.setItem('app.vault', JSON.stringify(vault));

		$vaultPicker.open = false;
		$cmdPal.open = false;
		goto('/editor');
	};
	const onNewVault = async () => {
		const selected = await open({
			multiple: false,
			directory: true
		});

		if (!selected) {
			return;
		}

		const vault = await invoke<VaultType>('add_vault', { path: selected });
		vaultPicker.update((ps) => ({ ...ps, vaults: [...ps.vaults, vault] }));
	};

	$: if (!$cmdPal.open) {
		$cmdPal.cmdInput = '';
		$cmdPal.fileInput = '';
	}
	$: if (!$vaultPicker.open) {
		$vaultPicker.value = undefined;
	}
</script>

<div class="app h-full">
	<ModeWatcher />

	{#if needs_auth_setup}
		<AuthSetup />
	{:else if $app.authed}
		<main class="h-full">
			<slot></slot>
		</main>
	{:else}
		<Login />
	{/if}

	<Command.Dialog
		bind:value={cmdValue}
		bind:open={$cmdPal.open}
		filter={(value, search) => {
			console.debug(value, search);
			// if ($cmdPal.files.length === 0 && $cmdPal.fileInput.trim().length > 0) {
			// 	return 1;
			// }
			// if ($cmdPal.element === 'filepicker') {
			// 	return value.includes(search.substring(3).trim()) ? 1 : 0;
			// }

			if (value.includes(search)) return 1;
			return 0;
		}}
		closeOnEscape
		closeOnOutsideClick
	>
		{#if $cmdPal.element === 'cmdpal'}
			<Command.Input
				placeholder="Type a command or search..."
				bind:value={$cmdPal.cmdInput}
				autofocus
			/>
			<Command.List>
				<Command.Empty>No results found.</Command.Empty>
				<Command.Group heading="Vaults">
					<Command.Item
						onSelect={() => {
							// $cmdPal.open = false;
							$vaultPicker.open = true;
						}}
					>
						<Settings class="mr-2 h-4 w-4" />
						<span>Open vault picker</span>
						<Command.Shortcut>{toString(openVault)}</Command.Shortcut>
					</Command.Item>
				</Command.Group>
				<Command.Separator />

				<Command.Group heading="Files">
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>New file</span>
						<Command.Shortcut>{toString(newFile)}</Command.Shortcut>
					</Command.Item>
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>Open vault file</span>
						<Command.Shortcut>{toString(openVaultFile)}</Command.Shortcut>
					</Command.Item>
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>Open file</span>
						<Command.Shortcut>⌘O</Command.Shortcut>
					</Command.Item>
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>Close file</span>
						<Command.Shortcut>⌘W</Command.Shortcut>
					</Command.Item>
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>Close all files</span>
						<Command.Shortcut>⌘⇧W</Command.Shortcut>
					</Command.Item>
				</Command.Group>
				<Command.Separator />

				<Command.Group heading="Settings">
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>Settings</span>
						<Command.Shortcut>⌘S</Command.Shortcut>
					</Command.Item>
				</Command.Group>

				<Command.Group heading="User">
					<Command.Item
						onSelect={async () => {
							$cmdPal.open = false;
							$vaultPicker.open = false;
							await invoke('logout');
							$app.authed = false;
						}}
					>
						<Settings class="mr-2 h-4 w-4" />
						<span>Logout</span>
						<Command.Shortcut>{toString(logout)}</Command.Shortcut>
					</Command.Item>
				</Command.Group>
			</Command.List>
		{:else if $cmdPal.element === 'filepicker'}
			<Command.Input placeholder="Search file..." bind:value={$cmdPal.fileInput} autofocus>
				<File class="mr-2 h-4 w-4 shrink-0 opacity-50" />
			</Command.Input>
			<Command.List>
				{#if $cmdPal.files.length > 0}
					<Command.Empty>No file found</Command.Empty>
					<Command.Group heading="Search files">
						{#each $cmdPal.files as f}
							<Command.Item>
								<!-- <Settings class="mr-2 h-4 w-4" /> -->
								<!-- TODO: fix this stupid fucker sorting only after 5chars ?????? WTF HALLO????!?? -->
								<span>{f}</span>
							</Command.Item>
						{/each}
					</Command.Group>
				{:else if $cmdPal.fileInput.trim().length > 0}
					<Command.Item>
						<FilePen class="mr-2 h-4 w-4" />
						<span>Create file: {$cmdPal.fileInput}</span>
					</Command.Item>
				{:else}
					<Command.Empty>You have 0 files</Command.Empty>
				{/if}
			</Command.List>
		{/if}
	</Command.Dialog>

	<Command.Dialog
		bind:open={$vaultPicker.open}
		bind:value={$vaultPicker.value}
		loop
		closeOnEscape
		closeOnOutsideClick
	>
		<Command.Input placeholder="Search vault" autofocus />
		<Command.List>
			<Command.Empty>No vaults found</Command.Empty>
			<Command.Group heading="Vaults">
				{#each $vaultPicker.vaults as vault}
					<Command.Item onSelect={() => onOpenVault(vault)}>
						<Vault class="mr-2 h-4 w-4" />
						<span>{vault.name}</span>
						<Command.Shortcut>{vault.path}</Command.Shortcut>
					</Command.Item>
				{/each}
			</Command.Group>
			<Command.Group heading="New vault">
				<Command.Item onSelect={onNewVault}>
					<Plus class="mr-2 h-4 w-4" />
					<span>Add a new vault</span>
				</Command.Item>
			</Command.Group>
		</Command.List>
	</Command.Dialog>

	<Toaster />
</div>

<style>
	.app {
		overflow: hidden;
	}
</style>
