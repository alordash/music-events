<script lang="ts">
	import type { ObjectExtractor } from '$lib/generic_object_form/FieldInfo';
	import IdDisplay from '$lib/generic_object_form/display/field_displays/IdDisplay.svelte';

	export let fieldName: string;
	export let value: number;

	export let objectExtractor: ObjectExtractor;

	let refObjectPromise = objectExtractor(value);
</script>

<label for="inputRef" class="col col-form-label">{fieldName}:</label>
<div class="col-8">
	<div class="input-group mb-3" id="inputRef">
		<span class="input-group-text">
			<IdDisplay id={value} />
		</span>
		<span class="input-group-text" data-bs-toggle="modal" data-bs-target="#exampleModal">
			{#await refObjectPromise}
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
			{:then refObject}
				{refObject?.name}
			{/await}
		</span>
	</div>
</div>
