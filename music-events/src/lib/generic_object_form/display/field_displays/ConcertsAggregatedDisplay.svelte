<script lang="ts">
	import { getEventConcerts, type Concert } from '$lib/model/concert/Concert';
	import ConcertDisplay from '$lib/model/concert/ConcertDisplay.svelte';

	export let eventId: number;
	let fieldName = 'Concerts';
	let collapseId = `${Math.random() % 1}`;

	let concertsPromise: Promise<Array<Concert>> = Promise.resolve([]);
	let collapsed = true;

	function onAccordionOpen() {
		if (collapsed) {
			concertsPromise = getEventConcerts(eventId);
		}
		collapsed = !collapsed;
	}
</script>

<div class="accordion">
	<div class="accordion-item">
		<h2 class="accordion-header">
			<button
				class="accordion-button collapsed"
				type="button"
				data-bs-toggle="collapse"
				data-bs-target="#collapse{collapseId}"
				aria-expanded="false"
				aria-controls="collapse{collapseId}"
				on:click={onAccordionOpen}
			>
				{fieldName}
			</button>
		</h2>
		<div id="collapse{collapseId}" class="accordion-collapse collapse">
			<div class="accordion-body">
				{#await concertsPromise}
					<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				{:then concerts}
					{#each concerts as concert}
						<ConcertDisplay {concert} short={true} showEditButton={false} />
					{/each}
				{/await}
			</div>
		</div>
	</div>
</div>
