<script lang="ts">
	import type { ClickCallback } from '$lib/generic_object_form/explorer/ClickCallback';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		fieldComposer,
		PARTICIPANT_ID_LITERAL,
		getParticipantsPaginated,
		getAllParticipants,

		getParticipantsCount

	} from './Participant';

	export let short = false;
	export let showEditButton = true;
	export let clickCallback: ClickCallback | undefined = undefined;
	export let pageCapacity = 6;
	export let columnsCount = 2;

	function objectExplorer(count: number, offset: number) {
		return getParticipantsPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	globalObjectExplorer={getAllParticipants}
	{objectExplorer}
	totalExplorer={getParticipantsCount}
	objectName="Participant"
	{fieldComposer}
	editLiteral={PARTICIPANT_ID_LITERAL}
	{pageCapacity}
	{columnsCount}
	{short}
	{showEditButton}
	{clickCallback}
/>
