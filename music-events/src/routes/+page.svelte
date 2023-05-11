<script lang="ts">
	import LoginForm from '$lib/main_page/LoginForm.svelte';
	import type { Concert } from '$lib/model/concert/Concert';
	import ConcertObjectDisplay from '$lib/model/concert/ConcertDisplay.svelte';
	let concert: Concert = {
		id: 1,
		address: 'aSd',
		date: '2222',
		durationMinutes: 344,
		name: 'qweqwe'
	};
</script>

<h1>Welcome to music events</h1>
<LoginForm />
