<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import TextDisplay from './field_displays/TextDisplay.svelte';
	import TimeMinutesDisplay from './field_displays/DurationMinutesDisplay.svelte';
	import type { FieldInfo, ObjectExtractor } from '../FieldInfo';
	import CostRublesDisplay from './field_displays/CostRublesDisplay.svelte';
	import ObjectRefDisplay from './field_displays/ObjectRefDisplay.svelte';
	import ConcertsAggregated from './field_displays/EventConcertsAggregatedDisplay.svelte';
	import ConcertGroupsAggregatedDisplay from './field_displays/ConcertGroupsAggregatedDisplay.svelte';
	import GroupArtistsAggregatedDisplay from './field_displays/GroupArtistsAggregatedDisplay.svelte';
	import ParticipantRepertoiresAggregatedDisplay from './field_displays/ParticipantRepertoiresAggregatedDisplay.svelte';

	export let fieldInfo: FieldInfo;
	export let value: any;
	export let objectId = 0;
	export let hideRefs = false;

	let fieldName = fieldInfo.fieldName;
	let fieldType = fieldInfo.fieldType;
	let objectExtractor = <ObjectExtractor>fieldInfo.objectExtractor;
</script>

{#if fieldType == FieldTypes.DurationMinutes}
	<div class="row"><TimeMinutesDisplay {fieldName} {value} /></div>
{:else if fieldType == FieldTypes.CostRubles}
	<div class="row"><CostRublesDisplay {fieldName} {value} /></div>
{:else if fieldType == FieldTypes.ObjectReference}
	<div class="row">
		<ObjectRefDisplay {fieldName} {value} {objectExtractor} nameComposer={fieldInfo.nameComposer} />
	</div>
{:else if fieldType == FieldTypes.GroupArtistsAggregated && !hideRefs}
	<GroupArtistsAggregatedDisplay groupId={objectId} />
{:else if fieldType == FieldTypes.ConcertGroupsAggregated && !hideRefs}
	<ConcertGroupsAggregatedDisplay concertId={objectId} />
{:else if fieldType == FieldTypes.EventConcertsAggregated && !hideRefs}
	<ConcertsAggregated eventId={objectId} />
{:else if fieldType == FieldTypes.ParticipantRepertoiresAggregated && !hideRefs}
	<ParticipantRepertoiresAggregatedDisplay participantId={objectId} />
{:else if fieldType == FieldTypes.Password}
	<!-- skip -->
{:else if fieldType == FieldTypes.Id}
	<!-- skip -->
{:else if fieldType == FieldTypes.Name}
	<!-- skip -->
{:else if fieldType == FieldTypes.Text}
	<div class="row"><TextDisplay {fieldName} {value} /></div>
{:else}
	<!-- skip -->
{/if}
