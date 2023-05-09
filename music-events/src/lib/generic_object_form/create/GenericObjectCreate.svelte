<script lang="ts">
	import { CreateState } from '$lib/CreateState';
	import type { GenericObject } from '../GenericObject';
	import GenericObjectCardHeader from '../GenericObjectCardHeader.svelte';
	import FieldEdit from '../edit/FieldEdit.svelte';
	import type { FieldInfo } from '../FieldInfo';

	export let createObject: GenericObject;
	export let objectName: string;
	export let fieldComposer: (fieldName: string) => FieldInfo;
	let infos = Object.keys(createObject)
		.map((key) => {
			return { key, fieldInfo: fieldComposer(key) };
		})
		.sort((a, b) => a.fieldInfo.priority - b.fieldInfo.priority);

	export let createCallback: (newObject: any) => Promise<number | undefined>;

	let createState = CreateState.Pending;
	let createObjectId: number | undefined = undefined;
	let errorMsg = '';

	async function onCreateButton() {
		try {
			createObjectId = await createCallback(createObject);
			createState = CreateState.Ok;
			if (createObject.id != undefined) {
				createObject.id = createObjectId;
			}
		} catch (error) {
			errorMsg = <string>error;
			createState = CreateState.Error;
		}
	}
</script>

<div class="card">
	<div class="card-body container">
		<GenericObjectCardHeader genericObject={createObject} {objectName} />

		{#each infos as info}
			<FieldEdit fieldInfo={info.fieldInfo} bind:value={createObject[info.key]} />
		{/each}
		<button type="submit" class="btn btn-primary" on:click={onCreateButton}>Create</button>
		<br />
		{#if createState == CreateState.Ok}
			<div
				class="p-2 mt-2 text-success-emphasis bg-success-subtle border border-success-subtle rounded-3"
			>
				Created {objectName}
				{#if createObjectId != null} with id: {createObjectId} {/if}
			</div>
		{:else if createState == CreateState.Error}
			<div
				class="p-2 mt-2 text-danger-emphasis bg-danger-subtle border border-danger-subtle rounded-3"
			>
				<b>Error creating new concert:</b>
				<br />
				{errorMsg}
			</div>
		{/if}
	</div>
</div>
