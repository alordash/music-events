<script lang="ts">
	import type { GenericObject } from '../GenericObject';
	import type { FieldTypes } from '../FieldTypes';
	import FieldDisplay from './FieldDisplay.svelte';
	import GenericObjectCardHeader from '../GenericObjectCardHeader.svelte';
	import { page } from '$app/stores';

	export let displayObject: GenericObject;
	export let objectName: string;
	export let fieldTypeExtractor: (fieldName: string) => FieldTypes;
	export let fieldNameFormatter: (key: string) => string;
	export let editLiteral: string | undefined;
	
	let fieldKeys = Object.keys(displayObject).map((key) => fieldTypeExtractor(key));

	const currentLink = $page.url.href;
	console.log('currentLink :>> ', currentLink);
	const editLink = currentLink.substring(0, currentLink.lastIndexOf('/')) + '/edit';
	console.log('editLink :>> ', editLink);
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
			<FieldDisplay fieldName={fieldNameFormatter(key)} fieldType={fieldKeys[i]} {value} />
		{/each}
	</div>
</div>
