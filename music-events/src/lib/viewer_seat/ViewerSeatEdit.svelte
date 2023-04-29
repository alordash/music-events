<script lang="ts">
	import { getAllConcertIdsAndNames, getConcertById, type Concert } from '$lib/concert/concert';
	import { transactionCommit, transactionRollback } from '$lib/general_controller';
	import { removeViewerSeatTransaction, type ViewerSeat } from './viewer_seat';

	export let viewerSeat: ViewerSeat;
	export let concertIdAndName: [number, string];
	export let changeCallback: (newViewerSeat: ViewerSeat) => void;
	export let deleteCallback: (deleteViewerSeat: ViewerSeat) => void;

	let deleteButton: HTMLButtonElement;
	let confirmDeleteButton: HTMLButtonElement;
	let cancelDeleteButton: HTMLButtonElement;

	let saveChangesButton: HTMLButtonElement | null;

	let hasChanges = false;
	let initialized = false;

	let deleting = false;
	let transactionId: object | null = null;

	let concertInfos: Array<[number, string]> = [];
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
		const concert = await getConcertById(concertId);
		console.log('Selected concert :>> ', concert);
		viewerSeat.concertId = <number>concert?.id;
		concertIdAndName[0] = viewerSeat.concertId;
		concertIdAndName[1] = <string>concert?.name;
	}

	async function onSaveChange() {
		changeCallback(viewerSeat);
		hasChanges = false;
	}

	async function onDelete() {
		deleting = true;
		transactionId = await removeViewerSeatTransaction(viewerSeat.id);
		console.log('Created transaction with id :>> ', transactionId);
	}

	async function onConfirmDelete() {
		deleting = false;
		if (transactionId == null) {
			return;
		}
		console.log('Commit transaction :>> ', transactionId);
		await transactionCommit(transactionId);
		transactionId = null;
		deleteCallback(viewerSeat);
	}

	async function onCancelDelete() {
		deleting = false;
		if (transactionId == null) {
			return;
		}
		console.log('Rollback transaction :>> ', transactionId);
		await transactionRollback(transactionId);
		transactionId = null;
	}

	$: {
		if (deleting) {
			deleteButton?.classList.remove('d-inline-block');
			deleteButton?.classList.add('d-none');

			confirmDeleteButton?.classList.remove('d-none');
			confirmDeleteButton?.classList.add('d-inline-block');

			cancelDeleteButton?.classList.remove('d-none');
			cancelDeleteButton?.classList.add('d-inline-block');
		} else {
			deleteButton?.classList.remove('d-none');
			deleteButton?.classList.add('d-inline-block');

			confirmDeleteButton?.classList.remove('d-inline-block');
			confirmDeleteButton?.classList.add('d-none');

			cancelDeleteButton?.classList.remove('d-inline-block');
			cancelDeleteButton?.classList.add('d-none');
		}
	}

	$: {
		viewerSeat = viewerSeat;
		if (initialized) {
			hasChanges = true;
		}
		initialized = true;
	}

	$: {
		if (hasChanges) {
			saveChangesButton?.classList.remove('disabled');
		} else {
			saveChangesButton?.classList.add('disabled');
		}
	}
</script>

<div>
	<div class="mb-3 row">
		<label for="staticId" class="col-sm-4 col-form-label">id:</label>
		<div class="col-sm-6">
			<input
				type="text"
				readonly
				class="form-control-plaintext"
				id="staticId"
				bind:value={viewerSeat.id}
			/>
		</div>
	</div>
	<div class="mb-3 row">
		<label for="inputKind" class="col-sm-4 col-form-label">kind:</label>
		<div class="col-sm-6">
			<input type="text" class="form-control" id="inputKind" bind:value={viewerSeat.kind} />
		</div>
	</div>
	<div class="mb-3 row">
		<label for="inputCostRubles" class="col-sm-4 col-form-label">Cost (rubles):</label>
		<div class="col-sm-6">
			<input
				type="number"
				class="form-control"
				id="inputCostRubles"
				bind:value={viewerSeat.costRubles}
			/>
		</div>
	</div>
	<div class="mb-3 row">
		<label for="inputRealNumber" class="col-sm-4 col-form-label">Number:</label>
		<div class="col-sm-6">
			<input
				type="number"
				class="form-control"
				id="inputRealNumber"
				bind:value={viewerSeat.realNumber}
			/>
		</div>
	</div>
	<div class="input-group mb-3">
		<span class="input-group-text">Concert:</span>
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

		<button
			class="btn btn-outline-secondary dropdown-toggle"
			type="button"
			data-bs-toggle="dropdown"
			aria-expanded="false"
		>
			<strong>{concertIdAndName[1]}</strong>
		</button>
	</div>
	<button
		type="submit"
		class="btn btn-primary"
		bind:this={saveChangesButton}
		on:click={onSaveChange}>Save changes</button
	>
	<button
		type="submit"
		class="btn btn-danger d-inline-block"
		bind:this={deleteButton}
		on:click={onDelete}>Delete</button
	>
	<button
		type="submit"
		class="btn btn-success d-none"
		bind:this={confirmDeleteButton}
		on:click={onConfirmDelete}>Confirm delete</button
	>
	<button
		type="submit"
		class="btn btn-danger d-none"
		bind:this={cancelDeleteButton}
		on:click={onCancelDelete}>Cancel</button
	>
</div>
