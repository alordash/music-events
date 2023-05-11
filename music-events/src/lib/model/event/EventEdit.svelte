<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Event } from './Event';

	export let event: Event;
	export let changeCallback: (newEvent: Event) => void;
	export let deleteCallback: (deleteEvent: Event) => void;
</script>

<GenericObjectEdit
	editObject={event}
	objectName="Event"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
