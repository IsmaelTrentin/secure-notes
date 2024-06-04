import { goto } from '$app/navigation';
import { invoke } from '@tauri-apps/api';
import { cmdPal } from './stores/cmdpal';

type Handler = (e: KeyboardEvent) => void | Promise<void>;

const bindings = new Map<string, Handler>();

const handleKeydown = (e: KeyboardEvent) => {
	const cmd = `${e.ctrlKey || e.metaKey ? '<s>' : ''}${e.key}`;
	console.debug('keybind:', cmd);
	const handler = bindings.get(cmd);
	handler && handler(e);
};

export function register(bind: string, handler: Handler) {
	bindings.set(bind, handler);
}
export function init() {
	document.addEventListener('keydown', handleKeydown);
	return () => {
		document.removeEventListener('keydown', handleKeydown);
	};
}

register('<s>P', (e) => {
	e.preventDefault();
	cmdPal.update((ps) => ({
		...ps,
		open: !ps.open || !(ps.element === 'cmdpal'),
		element: 'cmdpal'
	}));
});
register('<s>p', (e) => {
	e.preventDefault();
	cmdPal.update((ps) => ({
		...ps,
		open: !ps.open || !(ps.element === 'filepicker'),
		element: 'filepicker'
	}));
});
register('<s>n', async () => {
	await goto('/editor');
});
register('<s>l', async () => {
	await invoke<never>('logout');
});

const keybindsManager = { register, init };

export default keybindsManager;
