<script lang="ts">
	import File from 'lucide-svelte/icons/file';
	import Close from 'lucide-svelte/icons/x';
	import cx from 'classnames';
	import { app, type Buffer } from '$lib/stores/app';

	export let buffers: Buffer[];
	export let selectedId: string | undefined;

	const handleCloseBuffer = (buffer: Buffer) => {
		$app.openBuffers.delete(buffer.id);
		$app.selectedBufferId =
			$app.openBuffers.size !== 0 ? $app.openBuffers.keys().next().value : undefined;
	};
</script>

<div
	class="buffer-list flex h-10 w-full flex-row flex-nowrap items-start justify-start bg-zinc-800"
>
	{#each buffers as buffer}
		<button
			class={cx(
				'flex h-full max-w-56 select-none flex-row flex-nowrap items-center justify-center gap-1 overflow-hidden  border-solid border-teal-200 p-4',
				{
					'border-b-[1px]': selectedId === buffer.id
				},
				{
					'bg-zinc-700': selectedId === buffer.id
				}
			)}
			on:click={() => (selectedId = buffer.id)}
		>
			<File size={14} />
			<span>{buffer.filename}</span>
			<button on:click={() => handleCloseBuffer(buffer)}>
				<Close size={16} class="hover:stroke-zinc-200" />
			</button>
		</button>
	{/each}
</div>

<style>
	.buffer-list {
		grid-area: buffer-list;
	}
</style>
