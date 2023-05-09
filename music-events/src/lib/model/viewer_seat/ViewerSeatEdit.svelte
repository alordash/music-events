<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type ViewerSeat } from './ViewerSeat';

	export let viewerSeat: ViewerSeat;
	export let changeCallback: (newViewerSeat: ViewerSeat) => void;
	export let deleteCallback: (deleteViewerSeat: ViewerSeat) => void;
</script>

<GenericObjectEdit
	editObject={viewerSeat}
	objectName="Viewer seat"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
