<script lang="ts">
	import type { GenericObject } from '$lib/generic_object_form/GenericObject';
	import IdDisplay from '$lib/generic_object_form/display/field_displays/IdDisplay.svelte';
	import Concerts from '$lib/model/concert/Concerts.svelte';

	export let fieldName: string;
	export let value: number;

	export let objectExtractor: (id: number) => Promise<GenericObject>;

	const callback = (object: GenericObject | undefined) => {
		if (object == undefined) {
			return;
		}
		let id = object.id;
		if (id == undefined) {
			return;
		}
		value = id;
		refObjectPromise = Promise.resolve(object);
	};

	let refObjectPromise = objectExtractor(value);
</script>

<label for="inputRef" class="col col-form-label">{fieldName}:</label>
<div class="col-8 input-group mb-3" id="inputRef">
	<span class="input-group-text" id="basic-addon1">
		<IdDisplay id={value} />
	</span>
	<button
		type="button"
		class="btn btn-primary"
		data-bs-toggle="modal"
		data-bs-target="#exampleModal"
	>
		{#await refObjectPromise}
			<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		{:then refObject}
			{refObject.name}
		{/await}
	</button>
</div>

<div
	class="modal fade"
	id="exampleModal"
	tabindex="-1"
	aria-labelledby="exampleModalLabel"
	aria-hidden="true"
>
	<div class="modal-dialog modal-lg modal-dialog-centered">
		<div class="modal-content">
			<div class="modal-header">
				<h1 class="modal-title fs-5" id="exampleModalLabel">Modal title</h1>
				<button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
			</div>
			<div class="modal-body">
				<Concerts
					short={true}
					showEditButton={false}
					clickCallback={callback}
					pageCapacity={12}
					columnsCount={4}
				/>
			</div>
			<!-- <div class="modal-footer">
				<button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
			</div> -->
		</div>
	</div>
</div>
