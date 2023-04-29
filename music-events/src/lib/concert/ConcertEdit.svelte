<script lang="ts">
	import type { Concert } from './concert';

	export let concert: Concert;
	export let changeCallback: (newConcert: Concert) => void;
	export let deleteCallback: (deleteConcert: Concert) => void;

	let saveChangesButton: HTMLButtonElement | null;

	let hasChanges = false;
	let initialized = false;

	async function onSaveChange() {
		changeCallback(concert);
		hasChanges = false;
	}

	async function onDelete() {
		deleteCallback(concert);
	}

	$: {
		concert = concert;
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
				bind:value={concert.id}
			/>
		</div>
	</div>
	<div class="mb-3 row">
		<label for="inputName" class="col-sm-4 col-form-label">name:</label>
		<div class="col-sm-6">
			<input type="text" class="form-control" id="inputName" bind:value={concert.name} />
		</div>
	</div>
	<div class="mb-3 row">
		<label for="inputDate" class="col-sm-4 col-form-label">date:</label>
		<div class="col-sm-6">
			<input type="datetime-local" class="form-control" id="inputDate" bind:value={concert.date} />
		</div>
	</div>
	<div class="mb-3 row">
		<label for="inputDuration" class="col-sm-4 col-form-label">duration (minutes):</label>
		<div class="col-sm-6">
			<input
				type="number"
				class="form-control"
				id="inputDuration"
				bind:value={concert.durationMinutes}
			/>
		</div>
	</div>
	<div class="mb-3 row">
		<label for="inputAddress" class="col-sm-4 col-form-label">address:</label>
		<div class="col-sm-6">
			<input type="text" class="form-control" id="inputAddress" bind:value={concert.address} />
		</div>
	</div>
	<button
		type="submit"
		class="btn btn-primary"
		bind:this={saveChangesButton}
		on:click={onSaveChange}>Save changes</button
	>
	<button type="submit" class="btn btn-danger" on:click={onDelete}>Delete</button>
</div>
