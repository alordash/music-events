<script lang="ts">
	import { createEmpty, addEvent } from '$lib/model/event/Event';
	import EventObjectCreate from '$lib/model/event/EventCreate.svelte';
	let event = createEmpty();
</script>

<div class="w-50">
	<EventObjectCreate {event} createCallback={addEvent} />
</div>
