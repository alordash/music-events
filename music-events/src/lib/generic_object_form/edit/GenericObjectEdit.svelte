<script lang="ts">
	import type { FieldTypes } from '../FieldTypes';
	import type { GenericObject } from '../GenericObject';
	import GenericObjectCardHeader from '../GenericObjectCardHeader.svelte';
	import FieldEdit from './FieldEdit.svelte';

	export let editObject: GenericObject;
	export let objectName: string;
	export let fieldTypeExtractor: (fieldName: string) => FieldTypes;
	export let fieldNameFormatter: (key: string) => string;
	let fieldKeys = Object.keys(editObject);

	export let changeCallback: (newObject: any) => void;
	export let deleteCallback: (deleteObject: any) => void;

	let saveChangesButton: HTMLButtonElement | null;

	let hasChanges = false;
	let initialized = false;

	async function onSaveChange() {
		changeCallback(editObject);
		hasChanges = false;
	}

	async function onDelete() {
		deleteCallback(editObject);
	}

	$: {
		editObject = editObject;
		if (initialized) {
			hasChanges = true;
		}
		initialized = true;
	}

	$: {
		if (hasChanges) {
			saveChangesButton?.classList.remove('disabled');
		} else {
			saveChangesButton?.classList.add('disabled');
		}
	}
</script>

<div class="card">
	<div class="card-body container">
		<GenericObjectCardHeader genericObject={editObject} {objectName} />

		{#each Object.keys(editObject) as key, i}
			<FieldEdit
				fieldName={fieldNameFormatter(key)}
				fieldType={fieldTypeExtractor(fieldKeys[i])}
				bind:value={editObject[key]}
			/>
		{/each}
		<button
			type="submit"
			class="btn btn-primary"
			bind:this={saveChangesButton}
			on:click={onSaveChange}>Save changes</button
		>
		<button type="submit" class="btn btn-danger" on:click={onDelete}>Delete</button>
	</div>
</div>