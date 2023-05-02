<script lang="ts">
	import { page } from '$app/stores';
	import {
		getAllConcertIdsAndNames,
		getConcertById,
		removeConcert,
		updateConcert,
		type Concert
	} from '$lib/model/concert/Concert';
	import ConcertObjectEdit from '$lib/model/concert/ConcertObjectEdit.svelte';

	const concertIdLiteral = 'concert_id';

	let concert: Concert | null = null;

	let concertInfos: Array<[number, string]> = [];
	let initialized = false;
	async function init() {
		if (initialized) {
			return;
		}
		concertInfos = await getAllConcertIdsAndNames();
		console.log('concerts :>> ', concertInfos);
		initialized = true;
	}
	$: init();

	async function onSelectConcert(concertId: number) {
		$page.url.searchParams.set(concertIdLiteral, `${concertId}`);
		await loadConcert();
	}

	async function loadConcert() {
		const concertIdStr = $page.url.searchParams.get(concertIdLiteral);
		if (concertIdStr == null) {
			console.log(`Error getting concert id`);
			return;
		}
		const concertId = parseInt(concertIdStr);
		concert = await getConcertById(concertId);
		console.log('Selected concert :>> ', concert);
	}

	async function changeCallback(newConcert: Concert) {
		await updateConcert(newConcert);
	}

	async function deleteCallback(deleteConcert: Concert) {
		concert = null;
		const delId = concertInfos.findIndex(([i, _]) => i == deleteConcert.id);
		concertInfos.splice(delId, 1);
		concertInfos = concertInfos; // reassign to trigger reactive update
		await removeConcert(deleteConcert.id);
	}
</script>

<div class="dropdown">
	<button
		class="btn btn-secondary dropdown-toggle"
		type="button"
		data-bs-toggle="dropdown"
		aria-expanded="false"
	>
		Select concert
	</button>
	<ul class="dropdown-menu">
		{#each concertInfos as concertInfo}
			<li>
				<a
					class="dropdown-item"
					on:keypress={() => onSelectConcert(concertInfo[0])}
					on:click={() => onSelectConcert(concertInfo[0])}
					href="?{concertIdLiteral}={concertInfo[0]}">{concertInfo[1]}</a
				>
			</li>
		{/each}
	</ul>
</div>

{#if concert != null}
	<div class="w-50">
		<ConcertObjectEdit {concert} {changeCallback} {deleteCallback} />
	</div>
{/if}
