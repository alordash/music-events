<script lang="ts">
	import { page } from '$app/stores';
	import {
		getViewerSeatById,
		removeViewerSeat,
		updateViewerSeat,
		type ViewerSeat,
		VIEWER_SEAT_ID_LITERAL
	} from '$lib/model/viewer_seat/ViewerSeat';
	import ViewerSeatEdit from '$lib/model/viewer_seat/ViewerSeatEdit.svelte';

	let objectPromise: Promise<ViewerSeat | null> = Promise.resolve(null);

	async function loadConcert() {
		const idStr = $page.url.searchParams.get(VIEWER_SEAT_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting concert id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getViewerSeatById(id);
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
			<ViewerSeatEdit
				viewerSeat={object}
				changeCallback={updateViewerSeat}
				deleteCallback={(dc) => removeViewerSeat(dc.id)}
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
