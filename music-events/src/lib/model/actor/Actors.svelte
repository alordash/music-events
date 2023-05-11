<script lang="ts">
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldComposer,
		ACTOR_ID_LITERAL,
		getActorsPaginated,
		getActorsCount,
		nameComposer
	} from './Actor';

	function objectExplorer(count: number, offset: number) {
		return getActorsPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	{objectExplorer}
	totalCountExtractor={getActorsCount}
	objectName="Actor seat"
	{fieldComposer}
	editLiteral={ACTOR_ID_LITERAL}
	columnsCount={2}
	pageCapacity={6}
	{nameComposer}
/>
