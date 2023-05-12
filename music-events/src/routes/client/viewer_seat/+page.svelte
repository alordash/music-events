<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { CreateState } from '$lib/CreateState';
	import {
		addPerson,
		createEmpty,
		createPerson,
		getPersonByNameAndSurname,
		removePerson,
		type Person
	} from '$lib/model/person/Person';
	import PersonEdit from '$lib/model/person/PersonEdit.svelte';
	import {
		addUserPerson,
		createUserPerson,
		getUserPersonByUserIdAndPersonId,
		removeUserPerson
	} from '$lib/model/user_person/UserPerson';
	import { addViewer, createViewer, removeViewer, type Viewer } from '$lib/model/viewer/Viewer';
	import ViewerSeatDisplay from '$lib/model/viewer_seat/ViewerSeatDisplay.svelte';
	import { getCurrentAccount, roleMapper } from '$lib/user_forms/AccountStore';
	import {
		VIEWER_SEAT_ID_LITERAL,
		getViewerSeatById
	} from '../../../lib/model/viewer_seat/ViewerSeat';
	const idStr = $page.url.searchParams.get(VIEWER_SEAT_ID_LITERAL);
	const id = parseInt(idStr || '0');
	let viewerSeatPromise = getViewerSeatById(id);

	let createState = CreateState.Pending;
	let errorMsg = '';

	const HIDE = 'd-none';

	let person = createEmpty();

	let buyButton: HTMLButtonElement;
	let buyMoreButton: HTMLElement;
	let homeButton: HTMLElement;

	const currentUser = getCurrentAccount();

	function switchButtons() {
		buyButton.classList.add(HIDE);
		buyMoreButton.classList.remove(HIDE);
		homeButton.classList.remove(HIDE);
	}

	async function onBuyCallback() {
		let maybePersonId: number | null = null;
		let maybeViewerId: number | null = null;
		let maybeUserPersonId: number | null = null;
		let createdPerson = false;
		let createdUserPerson = false;
		try {
			let maybePerson = await getPersonByNameAndSurname(person.name, person.surname);
			if (maybePerson == null) {
				person = <Person>await createPerson(person);
				person.id = await addPerson(person);
				createdPerson = true;
			} else {
				person = maybePerson;
			}
			maybePersonId = person.id;
			let viewer = <Viewer>await createViewer({ personId: person.id, viewerSeatId: id, id: 0 });
			const viewerId = await addViewer(viewer);
			maybeViewerId = viewerId;
			let maybeUserPerson = await getUserPersonByUserIdAndPersonId(currentUser.id, person.id);
			if (maybeUserPerson == null) {
				let userPerson = await createUserPerson({
					id: 0,
					userId: currentUser.id,
					personId: person.id
				});
				createdPerson = true;
				maybeUserPersonId = await addUserPerson(userPerson);
			} else {
				maybeUserPersonId = maybeUserPerson.id;
			}
			createState = CreateState.Ok;
		} catch (error) {
			errorMsg = <string>error;
			createState = CreateState.Error;
			if (maybePersonId != null && createdPerson) {
				await removePerson(maybePersonId);
			}
			if (maybeViewerId != null) {
				await removeViewer(maybeViewerId);
			}
			if (maybeUserPersonId != null && createdUserPerson) {
				await removeUserPerson(maybeUserPersonId);
			}
		}
		if (createState == CreateState.Ok) {
			switchButtons();
		}
	}
</script>

{#await viewerSeatPromise}
	<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
{:then viewerSeat}
	{#if viewerSeat == null}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			<b>Error: viewer seat not found</b>
		</div>
	{:else}
		<div class="w-50">
			<ViewerSeatDisplay {viewerSeat} showEditButton={false} />
			<PersonEdit
				bind:person
				changeCallback={() => {}}
				deleteCallback={() => {}}
				showButtons={false}
				customTitle="Enter owner"
			/>
		</div>
		<button class="btn btn-success mt-1" bind:this={buyButton} on:click={onBuyCallback}>Buy</button>
		<a class="btn btn-info mt-1 d-none" bind:this={buyMoreButton} href="client/events">Buy more</a>
		<a
			class="btn btn-primary mt-1 d-none"
			bind:this={homeButton}
			href={roleMapper(currentUser.role)}>Home</a
		>
		{#if createState == CreateState.Ok}
			<div
				class="p-2 mt-2 text-success-emphasis bg-success-subtle border border-success-subtle rounded-3"
			>
				Successfully bought ticket.
			</div>
		{:else if createState == CreateState.Error}
			<div
				class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
			>
				<b>Error creating new concert:</b>
				<br />
				{errorMsg}
			</div>
		{/if}
	{/if}
{/await}
