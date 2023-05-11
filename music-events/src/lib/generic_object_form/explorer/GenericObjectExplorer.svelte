<script lang="ts">
	import { page } from '$app/stores';
	import { GenNumRange } from '$lib/Utils';
	import GenericObjectDisplay from '../display/GenericObjectDisplay.svelte';
	import type { FieldInfo, NameComposer, ObjectExplorer, TotalCountExtractor } from '../FieldInfo';
	import type { ClickCallback } from './ClickCallback';
	import { PAGE_LITERAL, type ExplorationResult } from './Paging';

	export let objectExplorer: ObjectExplorer;
	export let totalCountExtractor: TotalCountExtractor;
	export let objectName: string;
	export let fieldComposer: (fieldName: string) => FieldInfo;
	export let editLiteral: string | undefined;
	export let pageCapacity: number;

	export let currentPage: number = 0;
	export let extraPageButtonsCount: number = 2;
	export let columnsCount = 3;
	export let short = false;
	export let showEditButton = true;
	export let nameComposer: NameComposer | undefined = undefined;
	let currentOffset = 0;
	$: {
		let searchParamPage = $page.url.searchParams.get(PAGE_LITERAL);
		if (searchParamPage != null) {
			currentPage = parseInt(searchParamPage);
		}
	}

	$: {
		currentOffset = currentPage * pageCapacity;
	}
	export let clickCallback: ClickCallback | undefined = undefined;

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

	function formatObjectName() {
		let name = objectName;
		if (clickCallback == undefined) {
			name = `${name}s`;
		} else {
			name = `Select ${name}`;
		}
		return name;
	}
</script>

<div class="container">
	<div class="text-center card">
		<div class="card-header">
			<h4>
				{formatObjectName()}
				{#await totalCountAndPagesPromise then { totalPages }}
					{#if totalPages != 0}
						{#await currentObjectsPromise then { offset }}
							({Math.ceil(offset / pageCapacity) + 1}/{totalPages})
						{/await}
					{/if}
				{/await}
				{#await objectsPromise}
					<span
						class="spinner-border spinner-border-sm position-absolute mt-2 mx-1"
						role="status"
						aria-hidden="true"
					/>
				{/await}
			</h4>
		</div>
		{#await currentObjectsPromise}
			<div class="start-0 p-2">
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				Loading...
			</div>
		{:then { objects }}
			{#if objects.length == 0}
				Empty
			{:else}
				<div class="row row-cols-{columnsCount} text-start card-body">
					{#each objects as object}
						<div class="p-2">
							<GenericObjectDisplay
								displayObject={object}
								{objectName}
								{fieldComposer}
								{editLiteral}
								{short}
								{showEditButton}
								{clickCallback}
								{nameComposer}
							/>
						</div>
					{/each}
				</div>
			{/if}
		{/await}
		{#await totalCountAndPagesPromise}
			<div class="start-0 p-2">
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				Loading...
			</div>
		{:then { totalPages }}
			{#if totalPages > 1}
				<nav class="card-footer d-flex justify-content-center" aria-label="Pagination">
					<ul class="pagination m-0 me-2">
						<li class="page-item">
							<a class="page-link {currentPage == 0 ? 'disabled' : ''}" href="?{PAGE_LITERAL}={0}"
								>First</a
							>
						</li>
					</ul>
					<ul class="pagination justify-content-center m-0">
						{#if currentPage > extraPageButtonsCount}
							<li class="page-item">
								<a
									class="page-link {currentPage > 0 ? '' : 'disabled'}"
									href="?{PAGE_LITERAL}={currentPage - 1}">&laquo;</a
								>
							</li>
						{/if}
						{#each GenNumRange(currentPage - extraPageButtonsCount, currentPage) as prevPage}
							{#if prevPage >= 0 && prevPage < totalPages}
								<li class="page-item">
									<a class="page-link" href="?{PAGE_LITERAL}={prevPage}">{prevPage + 1}</a>
								</li>
							{/if}
						{/each}
						<li class="page-item disabled">
							<a class="page-link" href="?{PAGE_LITERAL}={currentPage}">{currentPage + 1}</a>
						</li>
						{#each GenNumRange(currentPage + 1, currentPage + extraPageButtonsCount + 1) as nextPage}
							{#if nextPage >= 0 && nextPage < totalPages}
								<li class="page-item">
									<a class="page-link" href="?{PAGE_LITERAL}={nextPage}">{nextPage + 1}</a>
								</li>
							{/if}
						{/each}
						{#if totalPages - currentPage > extraPageButtonsCount}
							<li class="page-item">
								<a
									class="page-link {currentPage < totalPages - 1 ? '' : 'disabled'}"
									href="?{PAGE_LITERAL}={currentPage + 1}">&raquo;</a
								>
							</li>
						{/if}
					</ul>

					<ul class="pagination m-0 ms-2">
						<li class="page-item">
							<a
								class="page-link {currentPage == totalPages - 1 ? 'disabled' : ''}"
								href="?{PAGE_LITERAL}={totalPages - 1}">Last</a
							>
						</li>
					</ul>
				</nav>
			{/if}
		{/await}
	</div>
</div>
