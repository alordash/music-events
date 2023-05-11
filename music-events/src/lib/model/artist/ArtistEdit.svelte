<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Artist } from './Artist';

	export let artist: Artist;
	export let changeCallback: (newArtist: Artist) => void;
	export let deleteCallback: (deleteArtist: Artist) => void;
</script>

<GenericObjectEdit
	editObject={artist}
	objectName="Artist seat"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
