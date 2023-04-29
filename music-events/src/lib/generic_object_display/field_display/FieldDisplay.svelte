<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import IdFieldDisplay from './IdDisplay.svelte';
	import TextDisplay from './TextDisplay.svelte';
	import TimeMinutesDisplay from './TimeMinutesDisplay.svelte';

	export let fieldName: string;
	export let value: unknown;
	export let fieldType: FieldTypes;
</script>

{#if fieldType == FieldTypes.TimeMinutes}
	<TimeMinutesDisplay {fieldName} {value} />
{:else if fieldType == FieldTypes.Text}
	<TextDisplay {fieldName} {value} />
{/if}
