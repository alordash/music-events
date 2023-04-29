<script lang="ts">
	import type { Concert } from './Concert';

	export let concert: Concert;
</script>

<div>
	<p>id: {concert.id}</p>
	<p>name: <strong>{concert.name}</strong></p>
	<p>date: {concert.date}</p>
	<p>duration (minutes): {concert.durationMinutes}</p>
	<p>address: {concert.address}</p>
</div>
