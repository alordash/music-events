<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Concert } from './Concert';

	export let concert: Concert;
	export let createCallback: (newConcert: Concert) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={concert} objectName="Concert" {fieldComposer} {createCallback} />
