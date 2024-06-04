import { writable } from 'svelte/store';
type Element = 'cmdpal' | 'filepicker';
type CmdPal = {
	open: boolean;
	element: Element;
	files: string[];
	cmdInput: string;
	fileInput: string;
};

function createCmdPal() {
	const { subscribe, set, update } = writable<CmdPal>({
		open: false,
		element: 'cmdpal',
		cmdInput: '',
		fileInput: '',
		files: ['dsad asd .ts']
	});

	return {
		subscribe,
		set,
		update
	};
}

export const cmdPal = createCmdPal();
