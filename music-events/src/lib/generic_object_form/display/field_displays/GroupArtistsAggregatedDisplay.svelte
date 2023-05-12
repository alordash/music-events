<script lang="ts">
	import ArtistDisplay from '$lib/model/artist/ArtistDisplay.svelte';
	import { type Artist, getGroupArtists } from '$lib/model/artist/Artist';

	export let groupId: number;
	let fieldName = 'Artists';
	let collapseId = `${Math.random() % 1}`;

	let artistsPromise: Promise<Array<Artist>> = Promise.resolve([]);
	let collapsed = true;

	function onAccordionOpen() {
		if (collapsed) {
			artistsPromise = getGroupArtists(groupId);
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
				{#await artistsPromise}
					<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				{:then artists}
					{#each artists as artist}
						<ArtistDisplay {artist} short={true} showEditButton={false} />
					{/each}
				{/await}
			</div>
		</div>
	</div>
</div>
