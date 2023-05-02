<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import {
		fieldNameFormatter,
		fieldTypeExtractor,
		CONCERT_ID_LITERAL,
		getAllConcerts
	} from './Concert';

	let concertsPromise = getAllConcerts();
</script>

<div class="container">
	<div class="text-center card">
		<div class="card-header">
			<h3>Concerts</h3>
		</div>
		{#await concertsPromise}
			<div class="start-0 p-2">
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
				Loading concerts...
			</div>
		{:then concerts}
			<div class="row row-cols-3 text-start card-body">
				{#each concerts as concert}
				<div class="p-2">
					<GenericObjectDisplay
						displayObject={concert}
						objectName="Concert"
						{fieldTypeExtractor}
						{fieldNameFormatter}
						editLiteral={CONCERT_ID_LITERAL}
					/>
				</div>
				{/each}
			</div>
		{/await}
	</div>
</div>
