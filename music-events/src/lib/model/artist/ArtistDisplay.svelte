<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import { fieldComposer, type Artist, ARTIST_ID_LITERAL, nameComposer } from './Artist';

	export let short = false;
	export let showEditButton = true;
	export let artist: Artist;
</script>

<GenericObjectDisplay
	displayObject={artist}
	objectName="Artist"
	{fieldComposer}
	editLiteral={ARTIST_ID_LITERAL}
	{nameComposer}
	{short}
	{showEditButton}
/>
