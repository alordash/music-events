<script lang="ts">
	import { page } from '$app/stores';
	import {
		getEventById,
		removeEvent,
		updateEvent,
		type Event,
		EVENT_ID_LITERAL
	} from '$lib/model/event/Event';
	import EventObjectEdit from '$lib/model/event/EventEdit.svelte';

	let objectPromise: Promise<Event | null> = Promise.resolve(null);

	async function loadEvent() {
		const idStr = $page.url.searchParams.get(EVENT_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting event id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getEventById(id);
	}

	loadEvent();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<EventObjectEdit
				event={object}
				changeCallback={updateEvent}
				deleteCallback={(dc) => removeEvent(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: event not found
		</div>
	{/if}
{/await}
