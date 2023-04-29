<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import IdFieldDisplay from './id_field/IdFieldDisplay.svelte';

	export let fieldName: string;
	export let value: unknown;
	export let fieldType: FieldTypes;
</script>

{#if fieldType == FieldTypes.Id}
	<div class="col"><IdFieldDisplay idValue={value} /></div>
{:else if fieldType == FieldTypes.Text}
	<div class="col">{fieldName}:</div>
	<div class="col">{value}</div>
{/if}
