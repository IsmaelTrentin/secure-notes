<script lang="ts">
	import { app } from '$lib/stores/app';
	import type { VaultEntry } from '$lib/stores/vault.picker';
	import cx from 'classnames';
	import * as Collapsible from '$lib/components/ui/collapsible/index.js';
	import { Button } from './ui/button';
	import { ChevronsUpDown } from 'lucide-svelte';

	export let children: VaultEntry[];
	export let selectedFilePath: string | undefined;
	export let nestLevel = 0;

	const handleOnClick = (file: VaultEntry) => {
		selectedFilePath = file.path;
		let openBufferId = undefined;

		for (const [id, buffer] of $app.openBuffers.entries()) {
			if (buffer.path === file.path) {
				openBufferId = id;
				break;
			}
		}

		if (openBufferId) {
			$app.selectedBufferId = openBufferId;
		} else {
			let id = Date.now().toString();
			$app.openBuffers.set(id, {
				id,
				...file
			});
			$app.selectedBufferId = id;
		}
	};

	// TODO: fix padding and dir with 0 children shown as files
</script>

<div
	class="flex flex-col flex-nowrap items-start justify-start gap-1"
	style={`padding-left: calc(0.1rem + ${Math.min(nestLevel, 1)}rem);`}
>
	{#each children as entry}
		{#if entry.children.length === 0}
			<button
				title={entry.path}
				class={cx('h-6 w-full text-left hover:text-zinc-200', {
					'bg-zinc-700': entry.path === (selectedFilePath ?? '')
				})}
				on:click={() => handleOnClick(entry)}
			>
				{entry.filename}
			</button>
		{:else}
			<Collapsible.Root class="w-full">
				<Collapsible.Trigger asChild let:builder>
					<Button
						builders={[builder]}
						variant="ghost"
						class={cx(
							'flex w-full flex-row items-center justify-start gap-1 !p-0 hover:text-zinc-200'
						)}
					>
						<ChevronsUpDown size={14} />
						<span class="sr-only">Toggle</span>
						<span>
							{entry.filename}
						</span>
					</Button>
				</Collapsible.Trigger>

				<Collapsible.Content class="space-y-2">
					<svelte:self children={entry.children} nestLevel={nestLevel + 1} />
				</Collapsible.Content>
			</Collapsible.Root>
		{/if}
	{/each}
</div>
