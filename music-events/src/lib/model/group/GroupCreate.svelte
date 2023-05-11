<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Group } from './Group';

	export let group: Group;
	export let createCallback: (newGroup: Group) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={group} objectName="Group" {fieldComposer} {createCallback} />
