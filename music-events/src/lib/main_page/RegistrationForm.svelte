<script lang="ts">
	import { CreateState } from '$lib/CreateState';
	import PasswordEdit from '$lib/generic_object_form/edit/field_edit/PasswordEdit.svelte';
	import TextEdit from '$lib/generic_object_form/edit/field_edit/TextEdit.svelte';
	import { addUser, getUserById, tryLogin, type User } from '$lib/model/user/User';

	let login = '';
	let password = '';

	let status = CreateState.Pending;
	let user: User | undefined = undefined;
	let errorMsg = '';
	async function onRegisterClick() {
		let userId: number;
		try {
			userId = await addUser({ login, password, role: 'client' });
			user = <User>await getUserById(userId);
			status = CreateState.Ok;
		} catch (error) {
			errorMsg = <string>error;
			status = CreateState.Error;
		}
	}
</script>

<div class="card w-50">
	<div class="card-body container">
		<h5 class="card-title">Register an account</h5>
	
		<TextEdit fieldName="Login" bind:value={login} />

		<PasswordEdit fieldName="Password" bind:value={password} />

		<div class="d-inline">
			<button class="btn btn-primary mt-3" on:click={onRegisterClick}>Register</button>
			<a class="btn btn-secondary mt-3" href={`../`}>Back</a>
		</div>

		{#if status == CreateState.Ok}
			<div
				class="p-2 mt-2 text-success-emphasis bg-success-subtle border border-success-subtle rounded-3"
			>
				Sucessfully registered {user?.login}
			</div>
		{:else if status == CreateState.Error}
			<div
				class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
			>
				Error: {errorMsg}
			</div>
		{/if}
	</div>
</div>
