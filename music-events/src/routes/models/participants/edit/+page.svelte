<script lang="ts">
	import { page } from '$app/stores';
	import {
		getParticipantById,
		removeParticipant,
		updateParticipant,
		type Participant,
		PARTICIPANT_ID_LITERAL
	} from '$lib/model/participant/Participant';
	import ParticipantObjectEdit from '$lib/model/participant/ParticipantEdit.svelte';

	let objectPromise: Promise<Participant | null> = Promise.resolve(null);

	async function loadParticipant() {
		const idStr = $page.url.searchParams.get(PARTICIPANT_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting participant id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getParticipantById(id);
	}

	loadParticipant();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<ParticipantObjectEdit
				participant={object}
				changeCallback={updateParticipant}
				deleteCallback={(dc) => removeParticipant(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: participant not found
		</div>
	{/if}
{/await}
