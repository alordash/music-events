<script lang="ts">
	import { page } from '$app/stores';
	import {
		getRepertoireById,
		removeRepertoire,
		updateRepertoire,
		type Repertoire,
		REPERTOIRE_ID_LITERAL
	} from '$lib/model/repertoire/Repertoire';
	import RepertoireObjectEdit from '$lib/model/repertoire/RepertoireEdit.svelte';

	let objectPromise: Promise<Repertoire | null> = Promise.resolve(null);

	async function loadRepertoire() {
		const idStr = $page.url.searchParams.get(REPERTOIRE_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting repertoire id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getRepertoireById(id);
	}

	loadRepertoire();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<RepertoireObjectEdit
				repertoire={object}
				changeCallback={updateRepertoire}
				deleteCallback={(dc) => removeRepertoire(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: repertoire not found
		</div>
	{/if}
{/await}
