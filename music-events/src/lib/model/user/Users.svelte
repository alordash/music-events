<script lang="ts">
	import type { ClickCallback } from '$lib/generic_object_form/explorer/ClickCallback';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldComposer,
		USER_ID_LITERAL,
		getUsersPaginated,
		getAllUsers,
		getUsersCount,
		nameComposer
	} from './User';

	export let short = false;
	export let showEditButton = true;
	export let clickCallback: ClickCallback | undefined = undefined;
	export let pageCapacity = 6;
	export let columnsCount = 2;

	function objectExplorer(count: number, offset: number) {
		return getUsersPaginated(count, offset).then((objects) => Promise.resolve({ objects, offset }));
	}
</script>

<GenericObjectExplorer
	globalObjectExplorer={getAllUsers}
	{objectExplorer}
	totalExplorer={getUsersCount}
	objectName="User"
	{fieldComposer}
	editLiteral={USER_ID_LITERAL}
	{pageCapacity}
	{columnsCount}
	{short}
	{showEditButton}
	{clickCallback}
	{nameComposer}
/>
