<script lang="ts">
	import '../app.pcss';
	import { ModeWatcher } from 'mode-watcher';
	import Login from '$lib/components/Login.svelte';
	import { listen } from '@tauri-apps/api/event';
	import { app } from '$lib/stores/app';
	import { invoke } from '@tauri-apps/api';

	const fetchAuthState = async () => {
		$app.loading = true;
		$app.authed = await invoke<boolean>('is_authenticated');
		$app.loading = false;
	};

	fetchAuthState();
	listen('auth_ok', async () => {
		$app.authed = true;
	});
	listen('logout', () => {
		$app.authed = false;
	});
</script>

<div class="app">
	<ModeWatcher />

	{#if $app.authed}
		<main>
			<slot></slot>
		</main>
	{:else}
		<Login />
	{/if}
</div>
