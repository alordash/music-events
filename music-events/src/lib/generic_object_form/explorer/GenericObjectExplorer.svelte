<script lang="ts">
	import { page } from '$app/stores';
	import { GenNumRange } from '$lib/Utils';
	import type { FieldTypes } from '../FieldTypes';
	import GenericObjectDisplay from '../display/GenericObjectDisplay.svelte';
	import { PAGE_LITERAL, type ExplorationResult } from './Paging';

	export let objectExplorer: (count: number, offset: number) => Promise<ExplorationResult>;
	export let totalCountExtractor: () => Promise<number>;
	export let objectName: string;
	export let fieldTypeExtractor: (fieldName: string) => FieldTypes;
	export let fieldNameFormatter: (key: string) => string;
	export let editLiteral: string | undefined;
	export let pageCapacity: number;

	export let currentPage: number = 0;
	export let extraPageButtonsCount: number = 2;
	let currentOffset = 0;
	$: {
		let searchParamPage = $page.url.searchParams.get(PAGE_LITERAL);
		if (searchParamPage != null) {
			currentPage = parseInt(searchParamPage);
		}
	}

	$: {
		currentOffset = currentPage * pageCapacity;
		console.log('currentPage :>> ', currentPage);
	}

	let totalCountAndPagesPromise = totalCountExtractor().then((totalCount) => {
		return { totalCount, totalPages: Math.ceil(totalCount / pageCapacity) };
	});
	let objectsPromise: Promise<ExplorationResult> = new Promise((_res, _rej) => {});
	let currentObjectsPromise: Promise<ExplorationResult> = new Promise((_res, _rej) => {});
	// used to disabled reactive change trigger

	function updateCurrentObjectsPromise() {
		objectsPromise.then((v) => {
			if (v.offset == currentPage * pageCapacity) {
				currentObjectsPromise = Promise.resolve(v);
			}
		});
	}
	$: {
		objectsPromise = objectExplorer(pageCapacity, currentOffset);
		updateCurrentObjectsPromise();
	}
</script>

<div class="container">
	<div class="text-center card">
		<div class="card-header">
			<h4>
				{objectName}s
				{#await objectsPromise}
					<span
						class="spinner-border spinner-border-sm position-absolute mt-2 mx-1"
						role="status"
						aria-hidden="true"
					/>
				{/await}
			</h4>
			{#await totalCountAndPagesPromise then { totalPages }}
				{#await currentObjectsPromise then { offset }}
					({Math.ceil(offset / pageCapacity) + 1}/{totalPages})
				{/await}
			{/await}
		</div>
		{#await currentObjectsPromise}
			<div class="start-0 p-2">
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				Loading...
			</div>
		{:then { objects }}
			<div class="row row-cols-3 text-start card-body">
				{#each objects as object}
					<div class="p-2">
						<GenericObjectDisplay
							displayObject={object}
							{objectName}
							{fieldTypeExtractor}
							{fieldNameFormatter}
							{editLiteral}
						/>
					</div>
				{/each}
			</div>
		{/await}
		{#await totalCountAndPagesPromise}
			<div class="start-0 p-2">
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				Loading...
			</div>
		{:then { totalPages }}
			<nav class="card-footer d-flex justify-content-center" aria-label="Page navigation example">
				<ul class="pagination m-0 me-2">
					<li class="page-item">
						<a class="page-link" href="?{PAGE_LITERAL}={0}">First</a>
					</li>
				</ul>
				<ul class="pagination justify-content-center m-0">
					<li class="page-item">
						<a
							class="page-link {currentPage > 0 ? '' : 'disabled'}"
							href="?{PAGE_LITERAL}={currentPage - 1}">&laquo;</a
						>
					</li>
					{#each GenNumRange(currentPage - extraPageButtonsCount, currentPage) as prevPage}
						{#if prevPage >= 0 && prevPage < totalPages}
							<a class="page-link" href="?{PAGE_LITERAL}={prevPage}">{prevPage + 1}</a>
						{/if}
					{/each}
					<li class="page-item disabled">
						<a class="page-link" href="?{PAGE_LITERAL}={currentPage}">{currentPage + 1}</a>
					</li>
					{#each GenNumRange(currentPage + 1, currentPage + extraPageButtonsCount + 1) as nextPage}
						{#if nextPage >= 0 && nextPage < totalPages}
							<a class="page-link" href="?{PAGE_LITERAL}={nextPage}">{nextPage + 1}</a>
						{/if}
					{/each}
					<li class="page-item">
						<a
							class="page-link {currentPage < totalPages - 1 ? '' : 'disabled'}"
							href="?{PAGE_LITERAL}={currentPage + 1}">&raquo;</a
						>
					</li>
				</ul>

				<ul class="pagination m-0 ms-2">
					<li class="page-item">
						<a class="page-link" href="?{PAGE_LITERAL}={totalPages - 1}">Last</a>
					</li>
				</ul>
			</nav>
		{/await}
	</div>
</div>