<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import { nameComposer } from '../viewer_seat/ViewerSeat';
	import { fieldComposer, type Viewer, VIEWER_ID_LITERAL } from './Viewer';

	export let viewer: Viewer;
</script>

<GenericObjectDisplay
	displayObject={viewer}
	objectName="Viewer"
	{fieldComposer}
	editLiteral={VIEWER_ID_LITERAL}
	{nameComposer}
/>
