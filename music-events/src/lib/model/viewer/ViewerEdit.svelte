<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Viewer } from './Viewer';

	export let viewer: Viewer;
	export let changeCallback: (newViewer: Viewer) => void;
	export let deleteCallback: (deleteViewer: Viewer) => void;
</script>

<GenericObjectEdit
	editObject={viewer}
	objectName="Viewer"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
