<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import { fieldComposer, type Person, PERSON_ID_LITERAL, nameComposer } from './Person';

	export let person: Person;
</script>

<GenericObjectDisplay
	displayObject={person}
	objectName="Person"
	{fieldComposer}
	editLiteral={PERSON_ID_LITERAL}
	{nameComposer}
/>
