<script lang="ts">
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldNameFormatter,
		fieldTypeExtractor,
		CONCERT_ID_LITERAL,
		getConcertsPaginated,
		getConcertsCount
	} from './Concert';
</script>

<GenericObjectExplorer
	objectExplorer={getConcertsPaginated}
	totalCountExtractor={getConcertsCount}
	objectName="Concert"
	{fieldTypeExtractor}
	{fieldNameFormatter}
	editLiteral={CONCERT_ID_LITERAL}
	pageCapacity={3}
/>
