<script lang="ts">
	import type { ClickCallback } from '$lib/generic_object_form/explorer/ClickCallback';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldComposer,
		VIEWER_ID_LITERAL,
		getViewersPaginated,
		getAllViewers,
		getViewersCount
	} from './Viewer';

	export let short = false;
	export let showEditButton = true;
	export let clickCallback: ClickCallback | undefined = undefined;
	export let pageCapacity = 6;
	export let columnsCount = 2;

	function objectExplorer(count: number, offset: number) {
		return getViewersPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	globalObjectExplorer={getAllViewers}
	{objectExplorer}
	totalExplorer={getViewersCount}
	objectName="Viewer"
	{fieldComposer}
	editLiteral={VIEWER_ID_LITERAL}
	{pageCapacity}
	{columnsCount}
	{short}
	{showEditButton}
	{clickCallback}
/>
