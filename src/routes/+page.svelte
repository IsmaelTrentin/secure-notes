<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import Sun from 'svelte-radix/Sun.svelte';
	import Moon from 'svelte-radix/Moon.svelte';
	import { toggleMode } from 'mode-watcher';
	import Label from '$lib/components/ui/label/label.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import { invoke } from '@tauri-apps/api';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';

	let text: string;

	const logout = async () => {
		await invoke<never>('logout');
	};
</script>

<div class="flex h-full flex-grow flex-row">
	<div class="w-60 min-w-60 bg-zinc-900">menu</div>
	<div class="flex h-full w-full flex-col">
		<div class="h-10 w-full bg-zinc-800">
			<Breadcrumb.Root>
				<Breadcrumb.List>
					<Breadcrumb.Item>
						<p>tu madre</p>
					</Breadcrumb.Item>
					<Breadcrumb.Separator />
					<Breadcrumb.Item>
						<p>culo</p>
					</Breadcrumb.Item>
				</Breadcrumb.List>
			</Breadcrumb.Root>
		</div>
		<div>
			<Label for="plaintext">Text</Label>
			<Textarea placeholder="Plaintext" id="plaintext" bind:value={text} />

			<a href="/login">login</a>
			<a href="/test">test</a>

			<Button on:click={toggleMode} variant="outline" size="icon">
				<Sun
					class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
				/>
				<Moon
					class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
				/>
				<span class="sr-only">Toggle theme</span>
			</Button>

			<Button on:click={logout}>Logout</Button>
		</div>
	</div>
</div>
