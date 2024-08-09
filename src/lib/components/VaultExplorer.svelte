<script lang="ts">
	import { app } from '$lib/stores/app';
	import FileTree from './FileTree.svelte';
	import { ScrollArea } from './ui/scroll-area';

	export let selectedFilePath: string | undefined;

	// TODO: fix colors, scroll area bar seethrough, handle not updating
</script>

<ScrollArea class="h-full w-full bg-zinc-900">
	<div class="p-4">{$app.vault?.name}</div>
	{#if $app.vault}
		<FileTree bind:selectedFilePath children={$app.vault?.entries} />
	{:else}
		<div></div>
	{/if}
</ScrollArea>
