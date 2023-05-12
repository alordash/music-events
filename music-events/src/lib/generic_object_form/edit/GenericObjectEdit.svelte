<script lang="ts">
	import type { FieldTypes } from '../FieldTypes';
	import type { GenericObject } from '../GenericObject';
	import GenericObjectCardHeader from '../GenericObjectCardHeader.svelte';
	import FieldEdit from './FieldEdit.svelte';
	import type { FieldInfo } from '../FieldInfo';

	export let editObject: GenericObject;
	export let objectName: string;
	export let fieldComposer: (fieldName: string) => FieldInfo;
	let infos = Object.keys(editObject)
		.map((key) => {
			return { key, fieldInfo: fieldComposer(key) };
		})
		.sort((a, b) => a.fieldInfo.priority - b.fieldInfo.priority);

	export let changeCallback: (newObject: any) => void;
	export let deleteCallback: (deleteObject: any) => void;

	export let showButtons = true;
	export let customTitle: string | undefined = undefined;

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
		<GenericObjectCardHeader genericObject={editObject} {objectName} {customTitle} />

		{#each infos as info}
			<FieldEdit fieldInfo={info.fieldInfo} bind:value={editObject[info.key]} />
		{/each}
		{#if showButtons}
			<button
				type="submit"
				class="btn btn-primary"
				bind:this={saveChangesButton}
				on:click={onSaveChange}>Save changes</button
			>
			<button type="submit" class="btn btn-danger" on:click={onDelete}>Delete</button>
		{/if}
	</div>
</div>
