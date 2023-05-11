<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Participant } from './Participant';

	export let participant: Participant;
	export let changeCallback: (newParticipant: Participant) => void;
	export let deleteCallback: (deleteParticipant: Participant) => void;
</script>

<GenericObjectEdit
	editObject={participant}
	objectName="Participant"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
