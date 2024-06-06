<script lang="ts">
	import { Carta, MarkdownEditor } from 'carta-md';
	import 'carta-md/default.css';

	const carta = new Carta({
		sanitizer: false
	});
</script>

<div class="flex h-full w-full">
	<div class="h-full w-[17rem] min-w-[17rem] overflow-y-auto">tree</div>
	<div class="h-full flex-grow">
		<div class="h-10 w-full"></div>
		<!-- <div
			class="editor-wrapper flex w-full items-start justify-center rounded-tl-lg bg-zinc-900 p-10 text-xl"
		> -->
		<div class="h-[100%]">
			<MarkdownEditor
				{carta}
				mode="tabs"
				disableToolbar
				placeholder="Enter text..."
				selectedTab="write"
				textarea={{ autoFocus: true }}
			/>
		</div>
		<!-- </div> -->
	</div>
</div>

<style>
	/* .editor-wrapper {
		height: calc(100vh - 2.5rem);
		border: 1px solid red !important;
		overflow: auto;
	} */
	:global(.carta-editor) {
		height: 100%;
		max-height: calc(100vh - 2.5rem);
		padding: 2.5rem;
		border: none !important;

		--tw-bg-opacity: 1;
		background-color: rgb(24 24 27 / var(--tw-bg-opacity));

		overflow-y: scroll;
	}
	/* :global(.carta-wrapper) { */
	/* flex-grow: 1; */
	/* } */
	/* :global(.carta-container) {
	} */
	:global(.carta-input) {
		overflow: hidden !important;
		height: 100% !important;
		height: calc(100vh - 2.5rem - 7.5rem) !important;
	}
	:global(.carta-font-code) {
		font-family: 'Fira Code', monospace;
		caret-color: white;
		font-size: 1.1rem;
	}
	:global(.carta-toolbar) {
		display: none;
	}
	/* Or in global stylesheet */
	/* Set your custom monospace font */
	:global(.carta-font-code) {
		font-family: ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji',
			'Segoe UI Symbol', 'Noto Color Emoji';
		font-size: 1.25rem /* 20px */;
		line-height: 1.75rem;
	}
</style>
