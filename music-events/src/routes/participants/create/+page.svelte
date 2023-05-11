<script lang="ts">
	import { createEmpty, addParticipant } from '$lib/model/participant/Participant';
	import ParticipantObjectCreate from '$lib/model/participant/ParticipantCreate.svelte';
	let participant = createEmpty();
</script>

<div class="w-50">
	<ParticipantObjectCreate {participant} createCallback={addParticipant} />
</div>
