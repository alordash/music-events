<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Viewer } from './Viewer';

	export let viewer: Viewer;
	export let createCallback: (newViewer: Viewer) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={viewer} objectName="Viewer" {fieldComposer} {createCallback} />
