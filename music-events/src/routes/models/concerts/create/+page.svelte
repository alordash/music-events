<script lang="ts">
	import { createEmpty, addConcert } from '$lib/model/concert/Concert';
	import ConcertObjectCreate from '$lib/model/concert/ConcertCreate.svelte';
	let concert = createEmpty();
</script>

<div class="w-50">
	<ConcertObjectCreate {concert} createCallback={addConcert} />
</div>
