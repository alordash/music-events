<script lang="ts">
	import type { DisplayableObject } from './DisplayableObject';
	import FieldDisplay from './field_display/FieldDisplay.svelte';

	export let displayObject: DisplayableObject;
	let fieldTypes = Object.keys(displayObject).map((key) => displayObject.getFieldType(key));
</script>

<div class="card" style="width: 18rem;">
	<div class="card-body container">
		<h5 class="card-title">
			{displayObject.constructor.name}
			{#if displayObject.getId != undefined}
				<span class="badge text-bg-info">#{displayObject.getId()}</span>
			{/if}
		</h5>
		{#each Object.entries(displayObject) as [key, value], i}
			<FieldDisplay
				fieldName={displayObject.formatFieldName(key)}
				{value}
				fieldType={fieldTypes[i]}
			/>
		{/each}
	</div>
</div>
