<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type GroupArtist } from './GroupArtist';

	export let groupArtist: GroupArtist;
	export let createCallback: (newGroupArtist: GroupArtist) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={groupArtist} objectName="GroupArtist" {fieldComposer} {createCallback} />
