<script lang="ts">
	import { page } from '$app/stores';
	import {
		getGroupById,
		removeGroup,
		updateGroup,
		type Group,
		GROUP_ID_LITERAL
	} from '$lib/model/group/Group';
	import GroupObjectEdit from '$lib/model/group/GroupEdit.svelte';

	let objectPromise: Promise<Group | null> = Promise.resolve(null);

	async function loadGroup() {
		const idStr = $page.url.searchParams.get(GROUP_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting group id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getGroupById(id);
	}

	loadGroup();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<GroupObjectEdit
				group={object}
				changeCallback={updateGroup}
				deleteCallback={(dc) => removeGroup(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: group not found
		</div>
	{/if}
{/await}
