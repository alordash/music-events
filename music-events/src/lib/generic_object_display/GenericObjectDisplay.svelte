<script lang="ts">
	import type { DisplayableObject } from './DisplayableObject';
	import type { FieldTypes } from './FieldTypes';
	import FieldDisplay from './field_display/FieldDisplay.svelte';
	import IdDisplay from './field_display/IdDisplay.svelte';

	export let displayObject: DisplayableObject;
	export let objectName: string;
	export let fieldTypeExtractor: (fieldName: string) => FieldTypes;
	export let fieldNameFormatter: (key: string) => string;
	let fieldTypes = Object.keys(displayObject).map((key) => fieldTypeExtractor(key));
</script>

<div class="card" style="width: 18rem;">
	<div class="card-body container">
		<h5 class="card-title user-select-all">
			{objectName}
			{#if displayObject.id != undefined}
				<IdDisplay id={displayObject.id} />
			{/if}
		</h5>
		{#each Object.entries(displayObject) as [key, value], i}
			<FieldDisplay fieldName={fieldNameFormatter(key)} {value} fieldType={fieldTypes[i]} />
		{/each}
	</div>
</div>
