<script lang="ts">
	import { page } from '$app/stores';
	import ViewerSeatEdit from '$lib/viewer_seat/ViewerSeatEdit.svelte';
	import {
		getAllViewerSeatIdsAndRealNumbersAndConcertNames,
		getViewerSeatById,
		updateViewerSeat,
		type ViewerSeat
	} from '$lib/viewer_seat/viewer_seat';

	const viewerSeatIdLiteral = 'viewer_seat_id';

	let viewerSeat: ViewerSeat | null = null;

	let viewerSeatInfos: Array<[number, number, string]> = [];
	let initialized = false;
	async function init() {
		if (initialized) {
			return;
		}
		viewerSeatInfos = await getAllViewerSeatIdsAndRealNumbersAndConcertNames();
		console.log('concerts :>> ', viewerSeatInfos);
		initialized = true;
	}
	$: init();

	async function onSelectConcert(viewerSeatId: number) {
		$page.url.searchParams.set(viewerSeatIdLiteral, `${viewerSeatId}`);
		await loadConcert();
	}

	async function loadConcert() {
		const viewerSeatIdStr = $page.url.searchParams.get(viewerSeatIdLiteral);
		if (viewerSeatIdStr == null) {
			console.log(`Error getting concert id`);
			return;
		}
		const viewerSeatId = parseInt(viewerSeatIdStr);
		viewerSeat = await getViewerSeatById(viewerSeatId);
		console.log('Selected concert :>> ', viewerSeat);
	}

	async function changeCallback(newViewerSeat: ViewerSeat) {
		await updateViewerSeat(newViewerSeat);
	}

	async function deleteCallback(deleteViewerSeat: ViewerSeat) {
		viewerSeat = null;
		const delId = viewerSeatInfos.findIndex(([i, _]) => i == deleteViewerSeat.id);
		viewerSeatInfos.splice(delId, 1);
		viewerSeatInfos = viewerSeatInfos; // reassign to trigger reactive update
	}

	function getCurrentConcertInfo() {
		const idx = viewerSeatInfos.findIndex(([i, _]) => i == viewerSeat?.id);
		return <[number, string]>[viewerSeatInfos[idx][1], viewerSeatInfos[idx][2]];
	}
</script>

<div class="dropdown">
	<button
		class="btn btn-secondary dropdown-toggle"
		type="button"
		data-bs-toggle="dropdown"
		aria-expanded="false"
	>
		Select viewer seat
	</button>
	<ul class="dropdown-menu">
		{#each viewerSeatInfos as viewerSeatInfo}
			<li>
				<a
					class="dropdown-item"
					on:keypress={() => onSelectConcert(viewerSeatInfo[0])}
					on:click={() => onSelectConcert(viewerSeatInfo[0])}
					href="?{viewerSeatIdLiteral}={viewerSeatInfo[0]}"
					>{`${viewerSeatInfo[2]}: ${viewerSeatInfo[1]}`}</a
				>
			</li>
		{/each}
	</ul>
</div>

{#if viewerSeat != null}
	<div class="card card-body">
		<ViewerSeatEdit
			{viewerSeat}
			concertIdAndName={getCurrentConcertInfo()}
			{changeCallback}
			{deleteCallback}
		/>
	</div>
{/if}
