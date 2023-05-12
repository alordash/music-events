<script lang="ts">
	import { getParticipantRepertoires, type Participant } from '$lib/model/participant/Participant';
	import type { Repertoire } from '$lib/model/repertoire/Repertoire';
	import RepertoireDisplay from '$lib/model/repertoire/RepertoireDisplay.svelte';

	export let participantId: number;
	let fieldName = 'Repertoires';
	let collapseId = `${Math.random() % 1}`;

	let repertoiresPromise: Promise<Array<Repertoire>> = Promise.resolve([]);
	let collapsed = true;

	function onAccordionOpen() {
		if (collapsed) {
			repertoiresPromise = getParticipantRepertoires(participantId);
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
				{#await repertoiresPromise}
					<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				{:then repertoires}
					{#each repertoires as repertoire}
						<RepertoireDisplay {repertoire} short={true} showEditButton={false} />
					{/each}
				{/await}
			</div>
		</div>
	</div>
</div>
