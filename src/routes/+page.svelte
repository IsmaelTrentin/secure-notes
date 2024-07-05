<script lang="ts">
	import {
		logout,
		newFile,
		openCmdPalette,
		openVault,
		openVaultFile,
		toString
	} from '$lib/keybinds';
</script>

<div class="flex h-full w-full flex-col items-center justify-center gap-5 bg-zinc-900">
	<div class="flex w-96 items-center justify-between">
		<h1 class="select-none text-2xl tracking-tight text-muted-foreground text-opacity-30">
			Command Palette
		</h1>
		<kbd
			class="pointer-events-none inline-flex h-10 select-none items-center gap-1 rounded border bg-muted px-2 pr-0 font-mono text-muted-foreground opacity-100"
		>
			<span class="text-2xl font-thin tracking-[.37rem]">{toString(openCmdPalette)}</span>
		</kbd>
	</div>

	<div class="flex w-96 items-center justify-between">
		<h1 class="select-none text-2xl tracking-tight text-muted-foreground text-opacity-30">
			New File
		</h1>
		<kbd
			class="pointer-events-none inline-flex h-10 select-none items-center gap-1 rounded border bg-muted px-2 pr-0 font-mono text-muted-foreground opacity-100"
		>
			<span class="text-2xl font-thin tracking-[.37rem]">{toString(newFile)}</span>
		</kbd>
	</div>

	<div class="flex w-96 items-center justify-between">
		<h1 class="select-none text-2xl tracking-tight text-muted-foreground text-opacity-30">
			Open Vault File
		</h1>
		<kbd
			class="pointer-events-none inline-flex h-10 select-none items-center gap-1 rounded border bg-muted px-2 pr-0 font-mono text-muted-foreground opacity-100"
		>
			<span class="text-2xl font-thin tracking-[.37rem]">{toString(openVaultFile)}</span>
		</kbd>
	</div>

	<div class="flex w-96 items-center justify-between">
		<h1 class="select-none text-2xl tracking-tight text-muted-foreground text-opacity-30">
			Open File
		</h1>
		<kbd
			class="pointer-events-none inline-flex h-10 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-muted-foreground opacity-100"
		>
			<span class="text-lg font-light">⌘</span>
			<span class="text-2xl font-thin">O</span>
		</kbd>
	</div>

	<div class="flex w-96 items-center justify-between">
		<h1 class="select-none text-2xl tracking-tight text-muted-foreground text-opacity-30">
			Open Vault
		</h1>
		<kbd
			class="pointer-events-none inline-flex h-10 select-none items-center gap-1 rounded border bg-muted px-2 pr-0 font-mono text-muted-foreground opacity-100"
		>
			<span class="text-2xl font-thin tracking-[.37rem]">{toString(openVault)}</span>
		</kbd>
	</div>

	<div class="flex w-96 items-center justify-between">
		<h1 class="select-none text-2xl tracking-tight text-muted-foreground text-opacity-30">
			Logout
		</h1>
		<kbd
			class="pointer-events-none inline-flex h-10 select-none items-center gap-1 rounded border bg-muted px-2 pr-0 font-mono text-muted-foreground opacity-100"
		>
			<span class="text-2xl font-thin tracking-[.37rem]">{toString(logout)}</span>
		</kbd>
	</div>
</div>

<!-- <Button on:click={toggleMode} variant="outline" size="icon">
	<Sun
		class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
	/>
	<Moon
		class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
	/>
	<span class="sr-only">Toggle theme</span>
</Button> -->

<!-- <Button on:click={logout}>Logout</Button> -->
