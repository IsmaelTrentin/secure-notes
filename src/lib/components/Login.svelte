<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { app } from '$lib/stores/app';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Form from '$lib/components/ui/form';

	import { Input } from '$lib/components/ui/input';
	import { z } from 'zod';
	import { defaults, setError, setMessage, superForm } from 'sveltekit-superforms';
	import { zod } from 'sveltekit-superforms/adapters';

	const formSchema = z.object({
		masterPw: z.string().min(1)
	});

	const form = superForm(defaults(zod(formSchema)), {
		SPA: true,
		validators: zod(formSchema),
		async onUpdate({ form, result }) {
			if (result.type === 'failure') {
				return;
			}

			$app.loading = true;
			try {
				await invoke<string>('authenticate', { masterPw: form.data.masterPw });
				setMessage(form, 'Auth ok');
			} catch (error) {
				setError(form, 'masterPw', error as unknown as string);
			}
			$app.loading = false;
		}
	});
	const { form: formData, enhance } = form;
</script>

<div class="h-full w-full">
	<div class="flex h-screen items-center justify-center">
		<Card.Root class="w-[450px] ">
			<Card.Header>
				<Card.Title tag="h1">Authenticate</Card.Title>
				<Card.Description>Authenticate using your master password.</Card.Description>
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

					<div class="mt-8"></div>
					<Form.Button class="w-full" disabled={$app.loading}>Submit</Form.Button>
				</form>
			</Card.Content>
		</Card.Root>
	</div>
</div>
