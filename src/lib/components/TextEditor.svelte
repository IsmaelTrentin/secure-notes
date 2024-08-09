<script lang="ts">
	import { app } from '$lib/stores/app';
</script>

<div class="text-editor h-full w-full bg-zinc-800 p-12">text editor {$app.selectedBufferId}</div>

<style>
	.text-editor {
		grid-area: text-editor;
	}
</style>
