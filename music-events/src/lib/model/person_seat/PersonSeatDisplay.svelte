<script lang="ts">
	import type { Person } from '../person/Person';
	import PersonDisplay from '../person/PersonDisplay.svelte';
	import { removeViewer, type Viewer } from '../viewer/Viewer';
	import type { ViewerSeat } from '../viewer_seat/ViewerSeat';
	import ViewerSeatDisplay from '../viewer_seat/ViewerSeatDisplay.svelte';

	export let person: Person;
	export let viewer: Viewer;
	export let viewerSeat: ViewerSeat;

	export let deleteCallback = (person: Person, viewer: Viewer, viewerSeat: ViewerSeat) => {};

	let returnButton: HTMLButtonElement;
	let confirmButton: HTMLButtonElement;
	let cancelButton: HTMLButtonElement;

	const HIDE = 'd-none';

	function showReturn() {
		returnButton.classList.remove(HIDE);
		confirmButton.classList.add(HIDE);
		cancelButton.classList.add(HIDE);
	}

	function hideReturn() {
		returnButton.classList.add(HIDE);
		confirmButton.classList.remove(HIDE);
		cancelButton.classList.remove(HIDE);
	}

	function onReturnClick() {
		hideReturn();
	}

	async function onConfirmClick() {
		await removeViewer(viewer.id);
		deleteCallback(person, viewer, viewerSeat);
		showReturn();
	}

	function onCancelClick() {
		showReturn();
	}
</script>

<div class="row">
	<div class="col border">
		<PersonDisplay {person} short={true} showEditButton={false} />
	</div>
	<div class="col border">
		<ViewerSeatDisplay {viewerSeat} short={true} showEditButton={false} />
		<button class="btn btn-warning" bind:this={returnButton} on:click={onReturnClick}>Return</button
		>
		<button class="btn btn-danger d-none" bind:this={confirmButton} on:click={onConfirmClick}
			>Confirm</button
		>
		<button class="btn btn-secondary d-none" bind:this={cancelButton} on:click={onCancelClick}
			>Cancel</button
		>
	</div>
</div>
