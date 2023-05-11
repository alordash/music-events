<script lang="ts">
	import { page } from '$app/stores';
	import {
		getArtistById,
		removeArtist,
		updateArtist,
		type Artist,
		ARTIST_ID_LITERAL
	} from '$lib/model/artist/Artist';
	import ArtistObjectEdit from '$lib/model/artist/ArtistEdit.svelte';

	let objectPromise: Promise<Artist | null> = Promise.resolve(null);

	async function loadArtist() {
		const idStr = $page.url.searchParams.get(ARTIST_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting artist id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getArtistById(id);
	}

	loadArtist();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<ArtistObjectEdit
				artist={object}
				changeCallback={updateArtist}
				deleteCallback={(dc) => removeArtist(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: artist not found
		</div>
	{/if}
{/await}
