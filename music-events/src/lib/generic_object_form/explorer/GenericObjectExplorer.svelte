<script lang="ts">
	import { page } from '$app/stores';
	import { GenNumRange } from '$lib/Utils';
	import GenericObjectDisplay from '../display/GenericObjectDisplay.svelte';
	import type { FieldInfo, NameComposer, ObjectExplorer, TotalCountExtractor } from '../FieldInfo';
	import type { GenericObject } from '../GenericObject';
	import type { ClickCallback } from './ClickCallback';
	import { PAGE_LITERAL, type ExplorationResult } from './Paging';

	export let objectExplorer: ObjectExplorer;
	export let totalExplorer: TotalCountExtractor;
	export let globalObjectExplorer: () => Promise<Array<GenericObject>>;
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
	export let reportMode = false;
	let currentOffset = 0;

	let oldPage = currentPage;
	let oldPageCapacity = pageCapacity;

	let searchName: string = '';

	function onReportModeChange() {
		if (reportMode) {
			currentPage = oldPage;
			pageCapacity = oldPageCapacity;
		} else {
			oldPage = currentPage;
			oldPageCapacity = pageCapacity;
			currentPage = 0;
			pageCapacity = Number.MAX_SAFE_INTEGER;
		}
		reportMode = !reportMode;
	}

	async function filterObj(obj: GenericObject) {
		let name = obj.name;
		if (name == null) {
			if (nameComposer == null) {
				return true;
			}
			name = await nameComposer(obj);
		}
		return name.includes(searchName);
	}

	async function filterObjs(objs: Array<GenericObject>): Promise<Array<GenericObject>> {
		let filteredObjects = [];
		for (const obj of objs) {
			if (await filterObj(obj)) {
				filteredObjects.push(obj);
			}
		}
		return filteredObjects;
	}

	let filteredExtractor = (count: number, offset: number) => {
		if (searchName != '') {
			return globalObjectExplorer().then(async (objects) => {
				return { objects: await filterObjs(objects), offset: pageCapacity };
			});
		}
		return objectExplorer(count, offset).then(async (result) => {
			return {
				objects: await filterObjs(result.objects),
				offset: result.offset
			};
		});
	};

	$: {
		let searchParamPage = $page.url.searchParams.get(PAGE_LITERAL);
		if (searchParamPage != null) {
			currentPage = parseInt(searchParamPage);
		}
	}

	$: {
		reportMode = reportMode;
	}

	$: {
		currentOffset = currentPage * pageCapacity;
	}
	export let clickCallback: ClickCallback | undefined = undefined;

	let totalCountAndPagesPromise = totalExplorer().then((totalCount) => {
		return { totalCount, totalPages: Math.ceil(totalCount / pageCapacity) };
	});

	let objectsPromise: Promise<ExplorationResult> = new Promise((_res, _rej) => {});
	let currentObjectsPromise: Promise<ExplorationResult> = new Promise((_res, _rej) => {});
	// used to disabled reactive change trigger

	function updateCurrentObjectsPromise() {
		objectsPromise.then((v) => {
			if (v.offset == currentPage * pageCapacity || searchName != '') {
				currentObjectsPromise = Promise.resolve(v);
			}
		});
	}

	$: {
		searchName = searchName;
		objectsPromise = filteredExtractor(pageCapacity, currentOffset);
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
					{#if totalPages != 0 && searchName == '' && !reportMode}
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
			<div class="mb-3 position-absolute start-0 w-25 top-0 p-2">
				<button
					class="input-group-text btn btn-info"
					id="search-addon"
					on:click={onReportModeChange}
					>{#if reportMode}Explore{:else}Form report{/if}</button
				>
			</div>
			{#if !reportMode}
				<div class="input-group mb-3 position-absolute end-0 w-25 top-0 p-2">
					<button class="input-group-text btn btn-primary" id="search-addon">Search</button>
					<input
						type="text"
						class="form-control"
						placeholder="Name"
						aria-label="Name"
						aria-describedby="search-addon"
						bind:value={searchName}
					/>
				</div>
			{/if}
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
								showEditButton={showEditButton && !reportMode}
								{clickCallback}
								{nameComposer}
								hideRefs={reportMode}
							/>
						</div>
					{/each}
				</div>
			{/if}
		{/await}
		{#if searchName == '' && !reportMode}
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
		{:else if reportMode}
			<nav class="card-footer d-flex justify-content-center" aria-label="Pagination" />
		{/if}
	</div>
</div>
