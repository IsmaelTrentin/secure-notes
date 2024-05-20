<script lang="ts">
	import '../app.pcss';
	import { ModeWatcher } from 'mode-watcher';
	import Login from '$lib/components/Login.svelte';
	import { listen } from '@tauri-apps/api/event';
	import { app } from '$lib/stores/app';
	import { invoke } from '@tauri-apps/api';
	import AuthSetup from '$lib/components/AuthSetup.svelte';

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
</script>

<div class="app">
	<ModeWatcher />

	{#if needs_auth_setup}
		<AuthSetup />
	{:else if $app.authed}
		<main>
			<slot></slot>
		</main>
	{:else}
		<Login />
	{/if}
</div>
