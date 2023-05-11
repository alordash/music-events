<script lang="ts">
	import type { NameComposer, ObjectExtractor } from '$lib/generic_object_form/FieldInfo';
	import IdDisplay from '$lib/generic_object_form/display/field_displays/IdDisplay.svelte';
	import NameDisplay from './NameDisplay.svelte';

	export let fieldName: string;
	export let value: number;

	export let objectExtractor: ObjectExtractor;

	export let nameComposer: NameComposer | undefined = undefined;

	let refObjectPromise = objectExtractor(value);
</script>

<label for="inputRef" class="col col-form-label">{fieldName}:</label>
<div class="col-8">
	<div class="input-group mb-3" id="inputRef">
		<span class="input-group-text">
			<IdDisplay id={value} />
		</span>
		<span class="input-group-text text-truncate">
			{#await refObjectPromise}
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />&nbsp;
			{:then refObject}
				{#if refObject == null}
					Not set
				{:else}
					<NameDisplay
						name={refObject.name}
						genericObject={refObject}
						{nameComposer}
						light={true}
					/>
				{/if}
			{/await}
		</span>
	</div>
</div>
