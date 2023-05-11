<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Repertoire } from './Repertoire';

	export let repertoire: Repertoire;
	export let createCallback: (newRepertoire: Repertoire) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={repertoire} objectName="Repertoire" {fieldComposer} {createCallback} />
