<script lang="ts">
	import { addConcert, addConcertTransaction, type Concert, createConcert } from '$lib/concert/Concert';
	import { CreateState } from '$lib/CreateState';
	import { transactionCommit, transactionRollback } from '$lib/GeneralController';

	let createButton: HTMLButtonElement;
	let confirmCreateButton: HTMLButtonElement;
	let cancelCreateButton: HTMLButtonElement;

	let createState = CreateState.Pending;
	let errorMsg = '';

	let concert = new Concert(0, '', 0, '', '');

	let creating = false;
	type transactionIdType = { concert: number };
	let transactionId: transactionIdType | null = null;
	let concertId: number | null = null;

	async function onCreate() {
		const concertObj = await createConcert(
			new Concert(0, concert.date, concert.durationMinutes, concert.address, concert.name)
		);
		console.log('concertObj :>> ', concertObj);
		[transactionId, concertId] = <[transactionIdType, number]>(
			await addConcertTransaction(concertObj)
		);
		creating = true;
		console.log('Created transaction with id :>> ', transactionId);
	}

	async function onConfirmCreate() {
		creating = false;
		if (transactionId == null) {
			return;
		}
		console.log('Commit transaction :>> ', transactionId);
		try {
			await transactionCommit(transactionId);
			transactionId = null;
			createState = CreateState.Ok;
		} catch (error) {
			createState = CreateState.Error;
			errorMsg = <string>error;
			console.log(`Error creating concert: `, error);
		}
	}

	async function onCancelCreate() {
		creating = false;
		if (transactionId == null) {
			return;
		}
		console.log('Rollback transaction :>> ', transactionId);
		await transactionRollback(transactionId);
		transactionId = null;
	}

	$: {
		if (creating) {
			createButton?.classList.remove('d-inline-block');
			createButton?.classList.add('d-none');

			confirmCreateButton?.classList.remove('d-none');
			confirmCreateButton?.classList.add('d-inline-block');

			cancelCreateButton?.classList.remove('d-none');
			cancelCreateButton?.classList.add('d-inline-block');
		} else {
			createButton?.classList.remove('d-none');
			createButton?.classList.add('d-inline-block');

			confirmCreateButton?.classList.remove('d-inline-block');
			confirmCreateButton?.classList.add('d-none');

			cancelCreateButton?.classList.remove('d-inline-block');
			cancelCreateButton?.classList.add('d-none');
		}
	}
</script>

<form>
	<h3>New concert</h3>
	<div class="mb-3">
		<label for="inputName" class="form-label">Name</label>
		<input type="text" bind:value={concert.name} class="form-control" id="inputName" />
	</div>
	<div class="mb-3">
		<label for="inputDate" class="form-label">Date</label>
		<input type="datetime-local" bind:value={concert.date} class="form-control" id="inputDate" />
	</div>
	<div class="mb-3">
		<label for="inputDuration" class="form-label">Duration (in minutes)</label>
		<input
			type="number"
			bind:value={concert.durationMinutes}
			class="form-control"
			id="inputDuration"
		/>
	</div>
	<div class="mb-3">
		<label for="inputAddress" class="form-label">Address</label>
		<input type="text" bind:value={concert.address} class="form-control" id="inputAddress" />
	</div>

	<button
		type="submit"
		class="btn btn-primary d-inline-block"
		bind:this={createButton}
		on:click={onCreate}>Create</button
	>
	<button
		type="submit"
		class="btn btn-success d-none"
		bind:this={confirmCreateButton}
		on:click={onConfirmCreate}>Confirm create</button
	>
	<button
		type="submit"
		class="btn btn-danger d-none"
		bind:this={cancelCreateButton}
		on:click={onCancelCreate}>Cancel</button
	>
	<br />
	<br />
	{#if createState == CreateState.Ok}
		<div class="p-3 text-success-emphasis bg-success-subtle border border-success-subtle rounded-3">
			Created concert with id: {concertId}
		</div>
	{:else if createState == CreateState.Error}
		<div class="p-3 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3">
			<b>Error creating new concert:</b>
			<br />
			{errorMsg}
		</div>
	{/if}
</form>
