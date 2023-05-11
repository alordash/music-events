<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Group } from './Group';

	export let group: Group;
	export let changeCallback: (newGroup: Group) => void;
	export let deleteCallback: (deleteGroup: Group) => void;
</script>

<GenericObjectEdit
	editObject={group}
	objectName="Group"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
