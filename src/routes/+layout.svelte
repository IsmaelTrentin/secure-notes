<script lang="ts">
	import '../app.pcss';
	import { ModeWatcher } from 'mode-watcher';
	import Login from '$lib/components/Login.svelte';
	import { listen } from '@tauri-apps/api/event';
	import { app } from '$lib/stores/app';
	import { invoke } from '@tauri-apps/api';
	import AuthSetup from '$lib/components/AuthSetup.svelte';
	import { onMount } from 'svelte';
	import keybindsManager from '$lib/keybinds';
	import * as Command from '$lib/components/ui/command';
	import Settings from 'lucide-svelte/icons/settings';
	import File from 'lucide-svelte/icons/file';
	import FilePen from 'lucide-svelte/icons/file-pen';
	import { cmdPal } from '$lib/stores/cmdpal';

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

	$: if (!open) {
		$cmdPal.cmdInput = '';
		$cmdPal.fileInput = '';
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
		bind:open={$cmdPal.open}
		filter={(value, search) => {
			if ($cmdPal.files.length === 0 && $cmdPal.fileInput.trim().length > 0) {
				return 1;
			}
			if ($cmdPal.element === 'filepicker') {
				return value.includes(search.substring(3).trim()) ? 1 : 0;
			}

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
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>Open vault</span>
						<Command.Shortcut>⌘O</Command.Shortcut>
					</Command.Item>
				</Command.Group>
				<Command.Separator />

				<Command.Group heading="Files">
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>New file</span>
						<Command.Shortcut>⌘N</Command.Shortcut>
					</Command.Item>
					<Command.Item>
						<Settings class="mr-2 h-4 w-4" />
						<span>Open file</span>
						<Command.Shortcut>⌘P</Command.Shortcut>
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
</div>

<style>
	.app {
		overflow: hidden;
	}
</style>
