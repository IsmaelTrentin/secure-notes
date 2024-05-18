<script>
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
</script>

<div class="app">
	<ModeWatcher />

	<main>
		<slot />
	</main>
</div>

<!-- <slot></slot> -->
