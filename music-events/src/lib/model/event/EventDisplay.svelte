<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import {
		fieldComposer,
		type Event,
		EVENT_ID_LITERAL
	} from './Event';

	export let event: Event;
</script>

<GenericObjectDisplay
	displayObject={event}
	objectName="Event"
	{fieldComposer}
	editLiteral={EVENT_ID_LITERAL}
/>
