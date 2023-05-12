<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Person } from './Person';

	export let person: Person;
	export let changeCallback: (newPerson: Person) => void;
	export let deleteCallback: (deletePerson: Person) => void;

	export let showButtons = true;
	export let customTitle: string | undefined = undefined;
</script>

<GenericObjectEdit
	editObject={person}
	objectName="Person"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
	{showButtons}
	{customTitle}
/>
