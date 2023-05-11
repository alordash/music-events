<script lang="ts">
	import { createEmpty, addActor } from '$lib/model/actor/Actor';
	import ActorObjectCreate from '$lib/model/actor/ActorCreate.svelte';
	let actor = createEmpty();
</script>

<div class="w-50">
	<ActorObjectCreate {actor} createCallback={addActor} />
</div>
