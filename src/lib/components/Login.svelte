<script lang="ts">
	// TODO: style component

	import Button from './ui/button/button.svelte';
	import { invoke } from '@tauri-apps/api';
	import { app } from '$lib/stores/app';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { z } from 'zod';

	export const formSchema = z.string().min(1);

	let masterPw: string;
	let errorMsg = '';

	const auth = async (e: MouseEvent) => {
		// e.preventDefault();

		try {
			formSchema.parse(masterPw);
		} catch (error) {
			console.log(error);

			const err = error as Zod.ZodError;
			errorMsg = err.errors[0].message;
			return;
		}

		$app.loading = true;
		await invoke<never>('authenticate');
		$app.loading = false;
	};

	const clearErrors = () => {
		errorMsg = '';
	};
</script>

<div class="h-full w-full">
	<div class="flex h-screen items-center justify-center">
		<Card.Root class="w-[450px] ">
			<Card.Header>
				<Card.Title tag="h1">Authenticate</Card.Title>
				<Card.Description>Authenticate using your master password.</Card.Description>
			</Card.Header>
			<Card.Content>
				<form>
					<div class="grid w-full items-center gap-4">
						<div class="flex flex-col space-y-1.5">
							<Label for="name">Master Password</Label>
							<Input
								id="name"
								placeholder="qwerty123"
								bind:value={masterPw}
								on:input={clearErrors}
							/>
							{#if errorMsg}
								<p class="text-sm text-red-500">{errorMsg}</p>
							{/if}
						</div>
					</div>
				</form>
			</Card.Content>
			<Card.Footer class="flex items-end">
				<Button on:click={auth} type="submit" disabled={$app.loading} class="w-full"
					>Authenticate</Button
				>
			</Card.Footer>
		</Card.Root>
	</div>
</div>
