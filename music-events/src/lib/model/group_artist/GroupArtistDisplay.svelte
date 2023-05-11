<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import {
		fieldComposer,
		type GroupArtist,
		GROUP_ARTIST_ID_LITERAL
	} from './GroupArtist';

	export let groupartist: GroupArtist;
</script>

<GenericObjectDisplay
	displayObject={groupartist}
	objectName="Group artist"
	{fieldComposer}
	editLiteral={GROUP_ARTIST_ID_LITERAL}
/>
