<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import {
		fieldComposer,
		type ViewerSeat,
		VIEWER_SEAT_ID_LITERAL,
		nameComposer
	} from './ViewerSeat';

	export let viewerSeat: ViewerSeat;
	export let short = false;
	export let showEditButton = true;
</script>

<GenericObjectDisplay
	displayObject={viewerSeat}
	objectName="Viewer seat"
	{fieldComposer}
	editLiteral={VIEWER_SEAT_ID_LITERAL}
	{nameComposer}
	{short}
	{showEditButton}
/>
