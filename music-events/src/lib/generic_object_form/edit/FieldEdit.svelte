<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import type {
		FieldComposer,
		FieldInfo,
		ObjectExplorer,
		ObjectExtractor,
		TotalCountExtractor
	} from '../FieldInfo';
	import DateEdit from './field_edit/DateEdit.svelte';
	import DurationMinutesEdit from './field_edit/DurationMinutesEdit.svelte';
	import NumberEdit from './field_edit/NumberEdit.svelte';
	import TextEdit from './field_edit/TextEdit.svelte';
	import ObjectRef from './field_edit/ObjectRefEdit.svelte';
	import CostRublesEdit from './field_edit/CostRublesEdit.svelte';
	import PasswordEdit from './field_edit/PasswordEdit.svelte';

	export let fieldInfo: FieldInfo;
	export let value: any;

	let fieldName = fieldInfo.fieldName;
	let fieldType = fieldInfo.fieldType;
	let objectExtractor = <ObjectExtractor>fieldInfo.objectExtractor;
	let objectExplorer = <ObjectExplorer>fieldInfo.objectExplorer;
	let totalCountExtractor = <TotalCountExtractor>fieldInfo.totalCountExtractor;
	let fieldComposer = <FieldComposer>fieldInfo.fieldComposer;
	let objectName = <string>fieldInfo.objectName;
</script>

{#if fieldType == FieldTypes.Id}
	<!-- skip -->
{:else if fieldType == FieldTypes.DurationMinutes}
	<div class="row"><DurationMinutesEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.Date}
	<div class="row"><DateEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.Number}
	<div class="row"><NumberEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.CostRubles}
	<div class="row"><CostRublesEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.Password}
	<div class="row"><PasswordEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.ObjectReference}
	<div class="row">
		<ObjectRef
			bind:value
			{fieldName}
			{objectExtractor}
			{objectExplorer}
			{totalCountExtractor}
			{fieldComposer}
			{objectName}
			nameComposer={fieldInfo.nameComposer}
		/>
	</div>
{:else if fieldType == FieldTypes.EventConcertsAggregated || fieldType == FieldTypes.ConcertGroupsAggregated}
	<!-- skip -->
{:else}
	<div class="row"><TextEdit bind:value {fieldName} /></div>
{/if}
