<script lang="ts">
	import { createEmpty, addPerson } from '$lib/model/person/Person';
	import PersonObjectCreate from '$lib/model/person/PersonCreate.svelte';
	let person = createEmpty();
</script>

<div class="w-50">
	<PersonObjectCreate {person} createCallback={addPerson} />
</div>
