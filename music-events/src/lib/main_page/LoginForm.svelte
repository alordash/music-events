<script lang="ts">
	import { goto } from '$app/navigation';
	import PasswordEdit from '$lib/generic_object_form/edit/field_edit/PasswordEdit.svelte';
	import TextEdit from '$lib/generic_object_form/edit/field_edit/TextEdit.svelte';
	import { tryLogin, type User } from '$lib/model/user/User';

	let login = '';
	let password = '';
	enum LoginStatus {
		Pending,
		Ok,
		Error
	}
	let status = LoginStatus.Pending;
	let user: User | undefined = undefined;
	async function onLoginClick() {
		user = <User>await tryLogin(login, password);
		if (user == undefined) {
			status = LoginStatus.Error;
		} else {
			status = LoginStatus.Ok;
		}

		switch (user.role) {
			case 'admin':
				goto('/admin/');
				break;
			case 'stuff':
				goto('/stuff/');
				break;
			case 'client':
				goto('/client/');
				break;
			default:
				break;
		}
	}
</script>

<div class="card w-50">
	<div class="card-body container">
		<h5 class="card-title">Login in account</h5>

		<TextEdit fieldName="Login" bind:value={login} />

		<PasswordEdit fieldName="Password" bind:value={password} />

		<div class="d-inline">
			<button class="btn btn-primary mt-3" on:click={onLoginClick}>Login</button>
			<a class="btn btn-info mt-3" href={`registration`}>Registration</a>
		</div>

		{#if status == LoginStatus.Ok}
			<div
				class="p-2 mt-2 text-success-emphasis bg-success-subtle border border-success-subtle rounded-3"
			>
				Sucessfully logged as {user?.login}
			</div>
		{:else if status == LoginStatus.Error}
			<div
				class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
			>
				Error: invalid credentials
			</div>
		{/if}
	</div>
</div>
