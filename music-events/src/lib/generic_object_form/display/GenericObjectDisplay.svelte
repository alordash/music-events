<script lang="ts">
	import type { GenericObject } from '../GenericObject';
	import type { FieldTypes } from '../FieldTypes';
	import FieldDisplay from './FieldDisplay.svelte';
	import GenericObjectCardHeader from '../GenericObjectCardHeader.svelte';
	import { page } from '$app/stores';
	import type { FieldInfo } from '../edit/FieldInfo';

	export let displayObject: GenericObject;
	export let objectName: string;
	export let fieldComposer: (fieldName: string) => FieldInfo;

	export let editLiteral: string | undefined;

	const currentLink = $page.url.href;
	const editLink = currentLink.substring(0, currentLink.lastIndexOf('/')) + '/edit';
</script>

<div class="card">
	<div class="card-body container">
		{#if displayObject.id != undefined && editLiteral != undefined}
			<a
				class="btn btn-primary position-absolute top-0 end-0 m-2"
				href={`${editLink}?${editLiteral}=${displayObject.id}`}
				role="button">Edit</a
			>
		{/if}

		<GenericObjectCardHeader genericObject={displayObject} {objectName} />
		{#each Object.entries(displayObject) as [key, value], i}
			<FieldDisplay fieldInfo={fieldComposer(key)} {value} />
		{/each}
	</div>
</div>
