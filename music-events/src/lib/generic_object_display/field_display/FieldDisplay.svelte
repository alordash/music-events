<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import TextDisplay from './TextDisplay.svelte';
	import TimeMinutesDisplay from './TimeMinutesDisplay.svelte';

	export let fieldName: string;
	export let value: unknown;
	export let fieldType: FieldTypes;
</script>

{#if fieldType == FieldTypes.TimeMinutes}
	<div class="row"><TimeMinutesDisplay {fieldName} {value} /></div>
{:else if fieldType == FieldTypes.Text}
	<div class="row"><TextDisplay {fieldName} {value} /></div>
{/if}
