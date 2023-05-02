<script lang="ts">
	import { createEmptyConcert, addConcert } from '$lib/model/concert/Concert';
	import ConcertObjectCreate from '$lib/model/concert/ConcertCreate.svelte';
	let concert = createEmptyConcert();

	async function onCreate() {
		let id = await addConcert(concert);
		console.log(`concert :>> ${concert}`, concert);
		return id;
	}
</script>

<div class="w-50">
	<ConcertObjectCreate {concert} createCallback={onCreate} />
</div>
