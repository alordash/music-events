<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import { fieldComposer, type Participant, PARTICIPANT_ID_LITERAL } from './Participant';

	export let participant: Participant;
</script>

<GenericObjectDisplay
	displayObject={participant}
	objectName="Participant"
	{fieldComposer}
	editLiteral={PARTICIPANT_ID_LITERAL}
/>
