<script lang="ts">
	import { createEmptyConcert, addConcert } from '$lib/model/concert/Concert';
	import ConcertObjectCreate from '$lib/model/concert/ConcertCreate.svelte';
	let concert = createEmptyConcert();
</script>

<div class="w-50">
	<ConcertObjectCreate {concert} createCallback={addConcert} />
</div>
