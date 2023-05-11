<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Artist } from './Artist';

	export let artist: Artist;
	export let createCallback: (newArtist: Artist) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={artist} objectName="Artist seat" {fieldComposer} {createCallback} />
