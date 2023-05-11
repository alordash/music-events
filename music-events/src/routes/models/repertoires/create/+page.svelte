<script lang="ts">
	import { createEmpty, addRepertoire } from '$lib/model/repertoire/Repertoire';
	import RepertoireObjectCreate from '$lib/model/repertoire/RepertoireCreate.svelte';
	let repertoire = createEmpty();
</script>

<div class="w-50">
	<RepertoireObjectCreate {repertoire} createCallback={addRepertoire} />
</div>
