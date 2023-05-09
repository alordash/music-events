<script lang="ts">
	import { page } from '$app/stores';
	import {
		getConcertById,
		removeConcert,
		updateConcert,
		type Concert,
		CONCERT_ID_LITERAL
	} from '$lib/model/concert/Concert';
	import ConcertObjectEdit from '$lib/model/concert/ConcertEdit.svelte';

	let objectPromise: Promise<Concert | null> = Promise.resolve(null);

	async function loadConcert() {
		const idStr = $page.url.searchParams.get(CONCERT_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting concert id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getConcertById(id);
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
			<ConcertObjectEdit
				concert={object}
				changeCallback={updateConcert}
				deleteCallback={(dc) => removeConcert(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: concert not found
		</div>
	{/if}
{/await}
