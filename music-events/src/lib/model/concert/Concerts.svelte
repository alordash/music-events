<script lang="ts">
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import type { ExplorationResult } from '$lib/generic_object_form/explorer/Paging';
	import {
		fieldNameFormatter,
		fieldTypeExtractor,
		CONCERT_ID_LITERAL,
		getConcertsPaginated,
		getConcertsCount
	} from './Concert';

	function objectExplorer(count: number, offset: number) {
		return getConcertsPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	{objectExplorer}
	totalCountExtractor={getConcertsCount}
	objectName="Concert"
	{fieldTypeExtractor}
	{fieldNameFormatter}
	editLiteral={CONCERT_ID_LITERAL}
	pageCapacity={3}
/>
