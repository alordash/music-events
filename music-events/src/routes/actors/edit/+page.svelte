<script lang="ts">
	import { page } from '$app/stores';
	import {
		getActorById,
		removeActor,
		updateActor,
		type Actor,
		ACTOR_ID_LITERAL
	} from '$lib/model/actor/Actor';
	import ActorObjectEdit from '$lib/model/actor/ActorEdit.svelte';

	let objectPromise: Promise<Actor | null> = Promise.resolve(null);

	async function loadActor() {
		const idStr = $page.url.searchParams.get(ACTOR_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting actor id`);
			return;
		}
		const id = parseInt(idStr);
		objectPromise = getActorById(id);
	}

	loadActor();
</script>

{#await objectPromise}
	<div class="p-2">
		<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		Loading...
	</div>
{:then object}
	{#if object != null}
		<div class="w-50">
			<ActorObjectEdit
				actor={object}
				changeCallback={updateActor}
				deleteCallback={(dc) => removeActor(dc.id)}
			/>
		</div>
	{:else}
		<div
			class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
		>
			Error: actor not found
		</div>
	{/if}
{/await}
