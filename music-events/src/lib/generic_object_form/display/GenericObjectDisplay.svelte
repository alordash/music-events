<script lang="ts">
	import type { GenericObject } from '../GenericObject';
	import FieldDisplay from './FieldDisplay.svelte';
	import GenericObjectCardHeader from '../GenericObjectCardHeader.svelte';
	import { page } from '$app/stores';
	import type { FieldInfo, NameComposer } from '../FieldInfo';
	import type { ClickCallback } from '../explorer/ClickCallback';

	export let displayObject: GenericObject;
	export let objectName: string;
	export let fieldComposer: (fieldName: string) => FieldInfo;
	export let short = false;
	export let showEditButton = true;
	let infos = Object.keys(displayObject)
		.map((key) => {
			return { key, fieldInfo: fieldComposer(key) };
		})
		.sort((a, b) => a.fieldInfo.priority - b.fieldInfo.priority);

	export let editLiteral: string | undefined;
	export let clickCallback: ClickCallback | undefined = undefined;
	export let nameComposer: NameComposer | undefined = undefined;
	const getAction = () =>
		clickCallback == undefined ? () => {} : () => (<ClickCallback>clickCallback)(displayObject);

	const currentLink = $page.url.href;
	const editLink = currentLink.substring(0, currentLink.lastIndexOf('/')) + '/edit';
</script>

<div class="card">
	<div
		class="card-body container {clickCallback == undefined ? '' : 'btn'}"
		data-bs-dismiss={clickCallback == undefined ? '' : 'modal'}
		on:click={getAction()}
		on:keydown={() => {}}
	>
		{#if showEditButton && displayObject.id != undefined && editLiteral != undefined}
			<a
				class="btn btn-primary position-absolute top-0 end-0 m-2"
				href={`${editLink}?${editLiteral}=${displayObject.id}`}
				role="button">Edit</a
			>
		{/if}

		<GenericObjectCardHeader genericObject={displayObject} {objectName} {short} {nameComposer} />
		{#if !short}
			{#each infos as info}
				<FieldDisplay fieldInfo={info.fieldInfo} value={displayObject[info.key]} />
			{/each}
		{/if}
	</div>
</div>
