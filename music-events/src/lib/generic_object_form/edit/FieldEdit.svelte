<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import DateEdit from './field_edit/DateEdit.svelte';
	import DurationMinutesEdit from './field_edit/DurationMinutesEdit.svelte';
	import TextEdit from './field_edit/TextEdit.svelte';

	export let fieldName: string;
	export let value: any;
	export let fieldType: FieldTypes;
</script>

{#if fieldType == FieldTypes.Id}
	<!-- skip -->
{:else if fieldType == FieldTypes.DurationMinutes}
	<div class="row"><DurationMinutesEdit bind:value {fieldName} /></div>
{:else if fieldType == FieldTypes.Date}
	<div class="row"><DateEdit bind:value {fieldName} /></div>
{:else}
	<div class="row"><TextEdit bind:value {fieldName} /></div>
{/if}
