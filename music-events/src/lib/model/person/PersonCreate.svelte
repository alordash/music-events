<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Person } from './Person';

	export let person: Person;
	export let createCallback: (newPerson: Person) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={person} objectName="Person" {fieldComposer} {createCallback} />
