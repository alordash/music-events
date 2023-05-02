<script lang="ts">
	import type { GenericObject } from '$lib/generic_object_form/GenericObject';
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldNameFormatter, fieldTypeExtractor, type Concert } from './Concert';

	export let concert: Concert;
	export let changeCallback: (newConcert: Concert) => void;
	export let deleteCallback: (deleteConcert: Concert) => void;
</script>

<GenericObjectEdit
	editObject={concert}
	objectName="Concert"
	{fieldTypeExtractor}
	{fieldNameFormatter}
	{changeCallback}
	{deleteCallback}
/>
