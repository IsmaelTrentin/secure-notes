<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { invoke } from '@tauri-apps/api';
	import { defaults, setError, setMessage, superForm } from 'sveltekit-superforms';
	import { zod } from 'sveltekit-superforms/adapters';
	import { z } from 'zod';

	let loading = false;
	const formSchema = z.object({
		masterPw: z.string().min(1),
		confirm: z.string().min(1)
	});

	const form = superForm(defaults(zod(formSchema)), {
		SPA: true,
		validators: zod(formSchema),
		async onUpdate({ form }) {
			if (form.data.masterPw !== form.data.confirm) {
				setError(form, 'confirm', 'Passwords must match');
			} else {
				console.log('submit?');
				setMessage(form, 'Password match!');
				loading = true;
				await invoke<never>('setup_auth', { masterPw: $formData.masterPw });
				loading = false;
			}
		}
	});
	const { form: formData, enhance } = form;
</script>

<div class="h-full w-full">
	<div class="flex h-screen items-center justify-center">
		<Card.Root class="w-[450px] ">
			<Card.Header>
				<Card.Title tag="h1">Master Password Setup</Card.Title>
				<Card.Description>Setup your master password for authentication.</Card.Description>
			</Card.Header>
			<Card.Content>
				<form method="POST" use:enhance>
					<Form.Field {form} name="masterPw">
						<Form.Control let:attrs>
							<Form.Label>Master Password</Form.Label>
							<Input {...attrs} bind:value={$formData.masterPw} type="password" />
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
					<Form.Field {form} name="confirm">
						<Form.Control let:attrs>
							<Form.Label>Confirm</Form.Label>
							<Input {...attrs} bind:value={$formData.confirm} type="password" />
						</Form.Control>
						<Form.FieldErrors />
					</Form.Field>
					<div class="mt-8"></div>
					<Form.Button class="w-full" disabled={loading}>Submit</Form.Button>
				</form>
			</Card.Content>
		</Card.Root>
	</div>
</div>
