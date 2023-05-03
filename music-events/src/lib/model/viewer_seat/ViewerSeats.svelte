<script lang="ts">
	import ViewerSeatDisplay from './ViewerSeatDisplay.svelte';
	import { getViewerSeatsPaginated, type ViewerSeat } from './ViewerSeat';

	let viewerSeats: Array<ViewerSeat> = [];

	let count = '3';
	let offset = '0';

	async function loadAllConcerts() {
		viewerSeats = await getViewerSeatsPaginated(+count, +offset);

		console.log('viewerSeats :>> ', viewerSeats);
	}
</script>

<div class="container">
	<div class="text-center card">
		<div class="card-header">
			<button on:click={loadAllConcerts} type="button" class="btn btn-primary">
				Get <input
					type="text"
					class="form-control d-inline-block"
					style="width: 2vw;"
					id="inputCount"
					bind:value={count}
				/>
				viewer seats, starting at
				<input
					type="text"
					class="form-control d-inline-block"
					style="width: 2vw;"
					id="inputOffset"
					bind:value={offset}
				/>
			</button>
		</div>
		{#if viewerSeats.length == 0}
			<div class="card-body">
				<p>No viewer seats.</p>
			</div>
		{:else}
			<div class="row row-cols-4 text-start card-body">
				{#each viewerSeats as viewerSeat}
					<div class="col border card-body">
						<ViewerSeatDisplay {viewerSeat} />
					</div>
				{/each}
			</div>
		{/if}
	</div>
</div>
