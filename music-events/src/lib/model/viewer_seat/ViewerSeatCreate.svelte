<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type ViewerSeat } from './ViewerSeat';

	export let viewerSeat: ViewerSeat;
	export let createCallback: (newViewerSeat: ViewerSeat) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={viewerSeat} objectName="Viewer seat" {fieldComposer} {createCallback} />
