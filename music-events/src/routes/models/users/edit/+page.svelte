<script lang="ts">
	import { page } from '$app/stores';
	import {
		getUserById,
		removeUser,
		updateUser,
		type User,
		USER_ID_LITERAL
	} from '$lib/model/user/User';
	import UserObjectEdit from '$lib/model/user/UserEdit.svelte';

	let objectPromise: Promise<User | null> = Promise.resolve(null);

	async function loadUser() {
		const idStr = $page.url.searchParams.get(USER_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting user id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getUserById(id);
	}

	loadUser();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<UserObjectEdit
				user={object}
				changeCallback={updateUser}
				deleteCallback={(dc) => removeUser(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: user not found
		</div>
	{/if}
{/await}
