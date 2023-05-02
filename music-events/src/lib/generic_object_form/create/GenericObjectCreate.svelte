<script lang="ts">
	import { CreateState } from '$lib/CreateState';
	import type { FieldTypes } from '../FieldTypes';
	import type { GenericObject } from '../GenericObject';
	import GenericObjectCardHeader from '../GenericObjectCardHeader.svelte';
	import FieldEdit from '../edit/FieldEdit.svelte';

	export let createObject: GenericObject;
	export let objectName: string;
	export let fieldTypeExtractor: (fieldName: string) => FieldTypes;
	export let fieldNameFormatter: (key: string) => string;
	let fieldKeys = Object.keys(createObject);

	export let createCallback: (newObject: any) => Promise<number | null>;

	let createState = CreateState.Pending;
	let createObjectId: number | null = null;
	let errorMsg = '';

	async function onCreateButton() {
		try {
			createObjectId = await createCallback(createObject);
			createState = CreateState.Ok;
		} catch (error) {
			errorMsg = <string>error;
			createState = CreateState.Error;
		}
	}
</script>

<div class="card">
	<div class="card-body container">
		<GenericObjectCardHeader genericObject={createObject} {objectName} />

		{#each Object.keys(createObject) as key, i}
			<FieldEdit
				fieldName={fieldNameFormatter(key)}
				fieldType={fieldTypeExtractor(fieldKeys[i])}
				bind:value={createObject[key]}
			/>
		{/each}
		<button type="submit" class="btn btn-primary" on:click={onCreateButton}>Create</button>
		<br />
		{#if createState == CreateState.Ok}
			<div
				class="p-3 mt-2 text-success-emphasis bg-success-subtle border border-success-subtle rounded-3"
			>
				Created {objectName}
				{#if createObjectId != null} with id: {createObjectId} {/if}
			</div>
		{:else if createState == CreateState.Error}
			<div class="p-3 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3">
				<b>Error creating new concert:</b>
				<br />
				{errorMsg}
			</div>
		{/if}
	</div>
</div>
