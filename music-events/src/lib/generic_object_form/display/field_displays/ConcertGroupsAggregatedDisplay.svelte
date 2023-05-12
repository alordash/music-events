<script lang="ts">
	import GroupDisplay from '$lib/model/group/GroupDisplay.svelte';
	import { getConcertGroups, type Group } from '$lib/model/group/Group';

	export let concertId: number;
	let fieldName = 'Groups';
	let collapseId = `${Math.random() % 1}`;

	let groupsPromise: Promise<Array<Group>> = Promise.resolve([]);
	let collapsed = true;

	function onAccordionOpen() {
		if (collapsed) {
			groupsPromise = getConcertGroups(concertId);
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
				{#await groupsPromise}
					<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				{:then groups}
					{#each groups as group}
						<GroupDisplay {group} short={true} showEditButton={false} />
					{/each}
				{/await}
			</div>
		</div>
	</div>
</div>
