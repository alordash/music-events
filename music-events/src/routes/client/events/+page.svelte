<script lang="ts">
	import { exploreComposer } from '$lib/generic_object_form/FieldInfo';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldComposer,
		getEventsCount,
		getEventsPaginated
	} from '../../../lib/model/event/Event';
	import type { GenericObject } from '$lib/generic_object_form/GenericObject';
	import { goto } from '$app/navigation';
	import { CONCERT_ID_LITERAL } from '$lib/model/concert/Concert';

	function callback(event: GenericObject | undefined) {
		goto(`/client/concerts?${CONCERT_ID_LITERAL}=${event?.id}`);
	}
</script>

<GenericObjectExplorer
	short={true}
	showEditButton={false}
	clickCallback={callback}
	pageCapacity={12}
	columnsCount={4}
	{fieldComposer}
	editLiteral=""
	objectExplorer={exploreComposer(getEventsPaginated)}
	objectName="event"
	totalExplorer={getEventsCount}
/>
