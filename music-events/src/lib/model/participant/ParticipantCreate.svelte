<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type Participant } from './Participant';

	export let participant: Participant;
	export let createCallback: (newParticipant: Participant) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={participant} objectName="Participant" {fieldComposer} {createCallback} />
