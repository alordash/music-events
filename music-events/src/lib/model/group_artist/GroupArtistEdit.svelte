<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type GroupArtist } from './GroupArtist';

	export let groupArtist: GroupArtist;
	export let changeCallback: (newGroupArtist: GroupArtist) => void;
	export let deleteCallback: (deleteGroupArtist: GroupArtist) => void;
</script>

<GenericObjectEdit
	editObject={groupArtist}
	objectName="GroupArtist"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
