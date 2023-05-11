<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Actor } from './Actor';

	export let actor: Actor;
	export let createCallback: (newActor: Actor) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={actor} objectName="Actor seat" {fieldComposer} {createCallback} />
