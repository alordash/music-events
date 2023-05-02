<script lang="ts">
	import type { GenericObject } from '../GenericObject';
	import type { FieldTypes } from '../FieldTypes';
	import FieldDisplay from './FieldDisplay.svelte';
	import IdDisplay from './field_displays/IdDisplay.svelte';

	export let displayObject: GenericObject;
	export let objectName: string;
	export let fieldTypeExtractor: (fieldName: string) => FieldTypes;
	export let fieldNameFormatter: (key: string) => string;
	let fieldKeys = Object.keys(displayObject).map((key) => fieldTypeExtractor(key));
</script>

<div class="card">
	<div class="card-body container">
		<h5 class="card-title user-select-all">
			{objectName}
			{#if displayObject.id != undefined}
				<IdDisplay id={displayObject.id} />
			{/if}
		</h5>
		{#each Object.entries(displayObject) as [key, value], i}
			<FieldDisplay fieldName={fieldNameFormatter(key)} fieldType={fieldKeys[i]} {value} />
		{/each}
	</div>
</div>
