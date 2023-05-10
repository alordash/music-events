<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import type { FieldInfo } from '../FieldInfo';
	import DateEdit from './field_edit/DateEdit.svelte';
	import DurationMinutesEdit from './field_edit/DurationMinutesEdit.svelte';
	import NumberEdit from './field_edit/NumberEdit.svelte';
	import TextEdit from './field_edit/TextEdit.svelte';
	import ObjectRef from './field_edit/ObjectRef.svelte';

	export let fieldInfo: FieldInfo;
	export let value: any;

	let fieldName = fieldInfo.fieldName;
	let fieldType = fieldInfo.fieldType;
	let objectExtractor = <any>fieldInfo.objectExtractor;
</script>

{#if fieldType == FieldTypes.Id}
	<!-- skip -->
{:else if fieldType == FieldTypes.DurationMinutes}
	<div class="row"><DurationMinutesEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.Date}
	<div class="row"><DateEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.Number}
	<div class="row"><NumberEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.ObjectReference}
	<div class="row">
		<ObjectRef bind:value {fieldName} {objectExtractor} />
	</div>
{:else}
	<div class="row"><TextEdit bind:value {fieldName} /></div>
{/if}
