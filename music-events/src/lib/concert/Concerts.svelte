<script lang="ts">
	import { getAllConcerts, type Concert } from './concert';
	import ConcertDisplay from './ConcertDisplay.svelte';

	let concerts: Array<Concert> = [];

	async function loadAllConcerts() {
		concerts = await getAllConcerts();

		console.log('concerts :>> ', concerts);
	}
</script>

<div class="container">
	<div class="text-center card">
		<div class="card-header">
			<button on:click={loadAllConcerts} type="button" class="btn btn-primary w-25">
				Get all concerts
			</button>
		</div>
		{#if concerts.length == 0}
			<div class="card-body">
				<p>No concerts.</p>
			</div>
		{:else}
			<div class="row row-cols-4 text-start card-body">
				{#each concerts as concert}
					<div class="col border card-body">
						<ConcertDisplay {concert} />
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
