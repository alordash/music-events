<script lang="ts">
	import { page } from '$app/stores';
	import {
		getViewerById,
		removeViewer,
		updateViewer,
		type Viewer,
		VIEWER_ID_LITERAL
	} from '$lib/model/viewer/Viewer';
	import ViewerObjectEdit from '$lib/model/viewer/ViewerEdit.svelte';

	let objectPromise: Promise<Viewer | null> = Promise.resolve(null);

	async function loadViewer() {
		const idStr = $page.url.searchParams.get(VIEWER_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting viewer id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getViewerById(id);
	}

	loadViewer();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<ViewerObjectEdit
				viewer={object}
				changeCallback={updateViewer}
				deleteCallback={(dc) => removeViewer(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: viewer not found
		</div>
	{/if}
{/await}
