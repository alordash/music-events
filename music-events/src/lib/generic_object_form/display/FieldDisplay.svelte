<script lang="ts">
	import { FieldTypes } from '../FieldTypes';
	import TextDisplay from './field_displays/TextDisplay.svelte';
	import TimeMinutesDisplay from './field_displays/DurationMinutesDisplay.svelte';

	export let fieldName: string;
	export let value: any;
	export let fieldType: FieldTypes;
</script>

{#if fieldType == FieldTypes.DurationMinutes}
	<div class="row"><TimeMinutesDisplay {fieldName} {value} /></div>
{:else}
	<div class="row"><TextDisplay {fieldName} {value} /></div>
{/if}
