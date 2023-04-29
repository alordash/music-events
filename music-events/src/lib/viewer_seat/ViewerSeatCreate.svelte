<script lang="ts">
	import { CreateState } from '$lib/CreateState';
	import { addViewerSeat, createViewerSeat, ViewerSeat } from './ViewerSeat';
	import { Decimal } from 'decimal.js';
	import { Concert, getAllConcertIdsAndNames, getConcertById } from '$lib/concert/Concert';

	let createState = CreateState.Pending;
	let newViewerSeatId = '';
	let errorMsg = '';

	let viewerSeat = new ViewerSeat(0, '', new Decimal(0), 0, 0);

	let concertInfos: Array<[number, string]> = [];
	let concert: Concert | null;

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
		await loadConcert(concertId);
	}

	async function loadConcert(concertId: number) {
		concert = await getConcertById(concertId);
		console.log('Selected concert :>> ', concert);
		viewerSeat.concertId = <number>concert?.id;
	}

	async function onCreate() {
		try {
			const viewerSeatObj = await createViewerSeat(
				new ViewerSeat(
					0,
					viewerSeat.kind,
					viewerSeat.costRubles,
					viewerSeat.realNumber,
					viewerSeat.concertId
				)
			);
			console.log('Creating viewer seat :>> ', viewerSeatObj);
			const viewerSeatId = await addViewerSeat(viewerSeatObj);
			console.log('New viewer seat id :>> ', viewerSeatId);
			createState = CreateState.Ok;
			newViewerSeatId = `${viewerSeatId}`;
		} catch (error) {
			createState = CreateState.Error;
			errorMsg = <string>error;
			console.log(`Error creating viewer seat: `, error);
		}
	}
</script>

<form>
	<h3>New viewer seat</h3>
	<div class="mb-3">
		<label for="inputKind" class="form-label">Kind</label>
		<input type="text" bind:value={viewerSeat.kind} class="form-control" id="inputName" />
	</div>
	<div class="mb-3">
		<label for="inputCostRubles" class="form-label">Cost (rubles)</label>
		<input
			type="number"
			bind:value={viewerSeat.costRubles}
			class="form-control"
			id="inputCostRubles"
		/>
	</div>
	<div class="mb-3">
		<label for="inputRealNumber" class="form-label">Number</label>
		<input
			type="number"
			bind:value={viewerSeat.realNumber}
			class="form-control"
			id="inputRealNumber"
		/>
	</div>

	<div class="input-group mb-3">
		<button
			class="btn btn-outline-secondary dropdown-toggle"
			type="button"
			data-bs-toggle="dropdown"
			aria-expanded="false"
		>
			Select concert
		</button>
		<div class="dropdown">
			<ul class="dropdown-menu">
				{#each concertInfos as concertInfo}
					<li>
						<div
							class="dropdown-item"
							on:keypress={() => onSelectConcert(concertInfo[0])}
							on:click={() => onSelectConcert(concertInfo[0])}
						>
							{concertInfo[1]}
						</div>
					</li>
				{/each}
			</ul>
		</div>
		<span class="input-group-text"
			>{#if concert == null}
				Not selected
			{:else}
				<strong>{concert.name}</strong>
			{/if}</span
		>
	</div>

	<br />
	<button type="submit" class="btn btn-primary" on:click={onCreate}>Create</button>
	<br />
	<br />
	{#if createState == CreateState.Ok}
		<div class="p-3 text-success-emphasis bg-success-subtle border border-success-subtle rounded-3">
			Created viewer seat with id: <b>{newViewerSeatId}</b>
		</div>
	{:else if createState == CreateState.Error}
		<div class="p-3 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3">
			<b>Error creating new viewer seat:</b>
			<br />
			{errorMsg}
		</div>
	{/if}
</form>
