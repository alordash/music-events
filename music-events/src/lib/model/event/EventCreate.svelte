<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Event } from './Event';

	export let event: Event;
	export let createCallback: (newEvent: Event) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={event} objectName="Event" {fieldComposer} {createCallback} />
