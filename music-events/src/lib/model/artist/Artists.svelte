<script lang="ts">
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldComposer,
		ARTIST_ID_LITERAL,
		getArtistsPaginated,
		nameComposer,
		getAllArtists,
		getArtistsCount
	} from './Artist';

	function objectExplorer(count: number, offset: number) {
		return getArtistsPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	globalObjectExplorer={getAllArtists}
	{objectExplorer}
	totalExplorer={getArtistsCount}
	objectName="Artist"
	{fieldComposer}
	editLiteral={ARTIST_ID_LITERAL}
	columnsCount={2}
	pageCapacity={6}
	{nameComposer}
/>
