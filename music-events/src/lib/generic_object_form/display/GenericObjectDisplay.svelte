<script lang="ts">
	import type { GenericObject } from '../GenericObject';
	import type { FieldTypes } from '../FieldTypes';
	import FieldDisplay from './FieldDisplay.svelte';
	import GenericObjectCardHeader from '../GenericObjectCardHeader.svelte';

	export let displayObject: GenericObject;
	export let objectName: string;
	export let fieldTypeExtractor: (fieldName: string) => FieldTypes;
	export let fieldNameFormatter: (key: string) => string;
	let fieldKeys = Object.keys(displayObject).map((key) => fieldTypeExtractor(key));
</script>

<div class="card">
	<div class="card-body container">
		<GenericObjectCardHeader genericObject={displayObject} {objectName} />
		{#each Object.entries(displayObject) as [key, value], i}
			<FieldDisplay fieldName={fieldNameFormatter(key)} fieldType={fieldKeys[i]} {value} />
		{/each}
	</div>
</div>
