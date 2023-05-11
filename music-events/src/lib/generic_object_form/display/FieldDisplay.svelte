<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import TextDisplay from './field_displays/TextDisplay.svelte';
	import TimeMinutesDisplay from './field_displays/DurationMinutesDisplay.svelte';
	import type { FieldInfo, ObjectExtractor } from '../FieldInfo';
	import CostRublesDisplay from './field_displays/CostRublesDisplay.svelte';
	import ObjectRefDisplay from './field_displays/ObjectRefDisplay.svelte';

	export let fieldInfo: FieldInfo;
	export let value: any;

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
{:else}
	<div class="row"><TextDisplay {fieldName} {value} /></div>
{/if}
