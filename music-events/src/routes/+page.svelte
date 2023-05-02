<script lang="ts">
	import type { Concert } from '$lib/model/concert/Concert';
	import ConcertObjectDisplay from '$lib/model/concert/ConcertObjectDisplay.svelte';
	let concert: Concert = {
		id: 1,
		address: 'aSd',
		date: '2222',
		durationMinutes: 344,
		name: 'qweqwe'
	};
</script>

<h1>Welcome to music events</h1>
<ConcertObjectDisplay {concert} />
