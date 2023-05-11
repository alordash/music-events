<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Repertoire } from './Repertoire';

	export let repertoire: Repertoire;
	export let changeCallback: (newRepertoire: Repertoire) => void;
	export let deleteCallback: (deleteRepertoire: Repertoire) => void;
</script>

<GenericObjectEdit
	editObject={repertoire}
	objectName="Repertoire"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
