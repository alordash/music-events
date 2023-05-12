<script lang="ts">
	import type { ClickCallback } from '$lib/generic_object_form/explorer/ClickCallback';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import { fieldComposer, EVENT_ID_LITERAL, getEventsPaginated, getAllEvents, getEventsCount } from './Event';

	export let short = false;
	export let showEditButton = true;
	export let clickCallback: ClickCallback | undefined = undefined;
	export let pageCapacity = 4;
	export let columnsCount = 2;

	function objectExplorer(count: number, offset: number) {
		return getEventsPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	globalObjectExplorer={getAllEvents}
	{objectExplorer}
	totalExplorer={getEventsCount}
	objectName="Event"
	{fieldComposer}
	editLiteral={EVENT_ID_LITERAL}
	{pageCapacity}
	{columnsCount}
	{short}
	{showEditButton}
	{clickCallback}
/>
