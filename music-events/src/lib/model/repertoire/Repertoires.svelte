<script lang="ts">
	import type { ClickCallback } from '$lib/generic_object_form/explorer/ClickCallback';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldComposer,
		REPERTOIRE_ID_LITERAL,
		getRepertoiresPaginated,
		getRepertoiresCount
	} from './Repertoire';

	export let short = false;
	export let showEditButton = true;
	export let clickCallback: ClickCallback | undefined = undefined;
	export let pageCapacity = 6;
	export let columnsCount = 2;

	function objectExplorer(count: number, offset: number) {
		return getRepertoiresPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	{objectExplorer}
	totalCountExtractor={getRepertoiresCount}
	objectName="Repertoire"
	{fieldComposer}
	editLiteral={REPERTOIRE_ID_LITERAL}
	{pageCapacity}
	{columnsCount}
	{short}
	{showEditButton}
	{clickCallback}
/>
