<script lang="ts">
	import { getConcertById, type Concert } from '$lib/concert/Concert';
	import ConcertDisplay from '$lib/concert/ConcertDisplay.svelte';
	import type { ViewerSeat } from './ViewerSeat';

	export let viewerSeat: ViewerSeat;

	const collapseId = `collapse${viewerSeat.id}`;

	let concert: Concert | null;

	async function loadConcert() {
		console.log('viewerSeat.concertId :>> ', viewerSeat.concertId);
		concert = await getConcertById(viewerSeat.concertId);
	}

	async function onConcertClick() {
		if (concert == null) {
			await loadConcert();
		}
	}

	async function onRefreshClick() {
        concert = null;
		await loadConcert();
	}
</script>

<div>
	<p>id: {viewerSeat.id}</p>
	<p>kind-number: <strong>{viewerSeat.kind}-{viewerSeat.realNumber}</strong></p>
	<p>cost (rubles): {viewerSeat.costRubles}</p>
	<div class="accordion" id="accordionExample">
		<div class="accordion-item">
			<h2 class="accordion-header">
				<button
					class="accordion-button collapsed"
					type="button"
					data-bs-toggle="collapse"
					data-bs-target="#{collapseId}"
					aria-expanded="false"
					aria-controls={collapseId}
					on:click={onConcertClick}
				>
					Concert
				</button>
			</h2>
			<div id={collapseId} class="accordion-collapse collapse" data-bs-parent="#accordionExample">
				<div class="accordion-body">
					<div class="container" style="position: relative;">
						{#if concert == null}
							<strong>Loading...</strong>
						{:else}
							<button
								type="button"
								class="btn btn-primary position-absolute top-0 end-0"
								on:click={onRefreshClick}><i class="bi bi-arrow-repeat" /></button
							>
							<ConcertDisplay {concert} />
						{/if}
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
