<script lang="ts">
	import { page } from '$app/stores';
	import {
		getGroupArtistById,
		removeGroupArtist,
		updateGroupArtist,
		type GroupArtist,
		GROUP_ARTIST_ID_LITERAL
	} from '$lib/model/group_artist/GroupArtist';
	import GroupArtistEdit from '$lib/model/group_artist/GroupArtistEdit.svelte';

	let objectPromise: Promise<GroupArtist | null> = Promise.resolve(null);

	async function loadConcert() {
		const idStr = $page.url.searchParams.get(GROUP_ARTIST_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting concert id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getGroupArtistById(id);
	}

	loadConcert();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<GroupArtistEdit
				groupArtist={object}
				changeCallback={updateGroupArtist}
				deleteCallback={(dc) => removeGroupArtist(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: viewer seat not found
		</div>
	{/if}
{/await}
