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

	let concertPromise: Promise<Concert | null> = Promise.resolve(null);

	async function loadConcert() {
		const concertIdStr = $page.url.searchParams.get(CONCERT_ID_LITERAL);
		if (concertIdStr == null) {
			console.log(`Error getting concert id`);
			return;
		}
		const concertId = parseInt(concertIdStr);
		concertPromise = getConcertById(concertId);
	}

	loadConcert();
</script>

{#await concertPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then concert}
	{#if concert != null}
		<div class="w-50">
			<ConcertObjectEdit
				{concert}
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
