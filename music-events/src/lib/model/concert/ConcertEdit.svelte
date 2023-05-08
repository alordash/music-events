<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Concert } from './Concert';

	export let concert: Concert;
	export let changeCallback: (newConcert: Concert) => void;
	export let deleteCallback: (deleteConcert: Concert) => void;
</script>

<GenericObjectEdit
	editObject={concert}
	objectName="Concert"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
