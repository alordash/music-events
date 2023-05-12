<script lang="ts">
	import PersonSeatDisplay from '$lib/model/person_seat/PersonSeatDisplay.svelte';
	import { createEmpty, getBoughtViewerSeats } from '$lib/model/user/User';
	import { accountStore, getCurrentAccount } from '$lib/user_forms/AccountStore';

	let user = createEmpty();
	accountStore.subscribe(() => (user = getCurrentAccount()));

	let personSeatPromise = getBoughtViewerSeats(user.id);
	const deleteCallback = () => {
		personSeatPromise = getBoughtViewerSeats(user.id);
	};
</script>

<h4>Your tickets</h4>

{#await personSeatPromise}
	<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
{:then personSeats}
	<div class="container">
		<div class="row">
			<div class="col">Owner</div>
			<div class="col">Ticket</div>
		</div>
		{#each personSeats as personSeat}
			<PersonSeatDisplay
				person={personSeat[0]}
				viewer={personSeat[1]}
				viewerSeat={personSeat[2]}
				{deleteCallback}
			/>
		{/each}
	</div>
{/await}
