<script lang="ts">
	import type { DisplayableObject } from './DisplayableObject';
	import FieldDisplay from './field_display/FieldDisplay.svelte';

	export let displayObject: DisplayableObject;
	let fieldTypes = Object.keys(displayObject).map((key) => displayObject.getFieldType(key));
</script>

<div class="card" style="width: 18rem;">
	<div class="card-body container">
		{#each Object.entries(displayObject) as [key, value], i}
			<div class="row">
				<FieldDisplay
					fieldName={displayObject.formatFieldName(key)}
					{value}
					fieldType={fieldTypes[i]}
				/>
			</div>
		{/each}
	</div>
</div>
