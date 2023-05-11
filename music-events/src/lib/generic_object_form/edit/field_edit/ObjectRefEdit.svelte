<script lang="ts">
	import type {
		FieldComposer,
		ObjectExplorer,
		ObjectExtractor,
		TotalCountExtractor
	} from '$lib/generic_object_form/FieldInfo';
	import type { GenericObject } from '$lib/generic_object_form/GenericObject';
	import IdDisplay from '$lib/generic_object_form/display/field_displays/IdDisplay.svelte';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';

	export let fieldName: string;
	export let value: number;

	export let objectExtractor: ObjectExtractor;
	export let fieldComposer: FieldComposer;
	export let objectExplorer: ObjectExplorer;
	export let totalCountExtractor: TotalCountExtractor;
	export let objectName: string;

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
<div class="col-8">
	<button
		class="input-group mb-3 border border-0 bg-transparent p-0"
		data-bs-toggle="modal"
		data-bs-target="#selectModal"
	>
		<span class="input-group-text w-auto">
			<IdDisplay id={value} />
		</span>
		<span class="input-group-text form-control">
			{#await refObjectPromise}
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />&nbsp;
			{:then refObject}
				{refObject == undefined ? 'Not set' : refObject.name}
			{/await}
		</span>
	</button>
</div>

<div class="modal fade" id="selectModal" tabindex="-1" aria-hidden="true">
	<div class="modal-dialog modal-lg modal-dialog-centered">
		<div class="modal-content">
			<div class="modal-body">
				<GenericObjectExplorer
					short={true}
					showEditButton={false}
					clickCallback={callback}
					pageCapacity={12}
					columnsCount={4}
					{fieldComposer}
					editLiteral=""
					{objectExplorer}
					{objectName}
					{totalCountExtractor}
				/>
			</div>
		</div>
	</div>
</div>
