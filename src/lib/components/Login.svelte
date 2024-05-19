<script lang="ts">
	// TODO: style component

	import Button from './ui/button/button.svelte';
	import { invoke } from '@tauri-apps/api';
	import { app } from '$lib/stores/app';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';

	const auth = async () => {
		$app.loading = true;
		await invoke<never>('authenticate');
		$app.loading = false;
	};
</script>

<div class="h-full w-full">
	<div class="flex h-screen items-center justify-center">
		<Card.Root class="h-[450px] w-[450px]">
			<Card.Header>
				<Card.Title tag="h1">Authenticate</Card.Title>
				<Card.Description>Authenticate using your master password.</Card.Description>
			</Card.Header>
			<Card.Content>
				<form>
					<div class="grid w-full items-center gap-4">
						<div class="flex flex-col space-y-1.5">
							<Label for="name">Master Password</Label>
							<Input id="name" placeholder="qwerty123" />
						</div>
					</div>
				</form>
			</Card.Content>
			<Card.Footer class="flex items-end">
				<Button on:click={auth} disabled={$app.loading}>Authenticate</Button>
			</Card.Footer>
		</Card.Root>
	</div>
</div>
