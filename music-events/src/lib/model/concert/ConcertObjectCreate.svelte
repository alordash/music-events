<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldNameFormatter, fieldTypeExtractor, type Concert } from './Concert';

	export let concert: Concert;
	export let createCallback: (newConcert: Concert) => Promise<number | null>;
</script>

<GenericObjectCreate
	createObject={concert}
	objectName="Concert"
	{fieldTypeExtractor}
	{fieldNameFormatter}
	{createCallback}
/>
