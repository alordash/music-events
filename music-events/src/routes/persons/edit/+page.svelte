<script lang="ts">
	import { page } from '$app/stores';
	import {
		getPersonById,
		removePerson,
		updatePerson,
		type Person,
		PERSON_ID_LITERAL
	} from '$lib/model/person/Person';
	import PersonObjectEdit from '$lib/model/person/PersonEdit.svelte';

	let objectPromise: Promise<Person | null> = Promise.resolve(null);

	async function loadPerson() {
		const idStr = $page.url.searchParams.get(PERSON_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting person id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getPersonById(id);
	}

	loadPerson();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<PersonObjectEdit
				person={object}
				changeCallback={updatePerson}
				deleteCallback={(dc) => removePerson(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: person not found
		</div>
	{/if}
{/await}
