<script lang="ts">
	import type { ClickCallback } from '$lib/generic_object_form/explorer/ClickCallback';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldComposer,
		GROUP_ARTIST_ID_LITERAL,
		getGroupArtistsPaginated,
		getAllGroupArtists,
		getGroupArtistsCount
	} from './GroupArtist';

	export let short = false;
	export let showEditButton = true;
	export let clickCallback: ClickCallback | undefined = undefined;
	export let pageCapacity = 3;
	export let columnsCount = 3;

	function objectExplorer(count: number, offset: number) {
		return getGroupArtistsPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	globalObjectExplorer={getAllGroupArtists}
	{objectExplorer}
	totalExplorer={getGroupArtistsCount}
	objectName="Group artist"
	{fieldComposer}
	editLiteral={GROUP_ARTIST_ID_LITERAL}
	{pageCapacity}
	{columnsCount}
	{short}
	{showEditButton}
	{clickCallback}
/>
