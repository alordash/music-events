<script lang="ts">
	import type { GenericObject } from './GenericObject';
	import IdDisplay from './display/field_displays/IdDisplay.svelte';
	import NameDisplay from './display/field_displays/NameDisplay.svelte';

	export let genericObject: GenericObject;
	export let objectName: string;
	export let short = false;
</script>

<h5 class="card-title user-select-all">
	{#if genericObject.name != undefined}
		<NameDisplay name={genericObject.name} />
	{/if}
	{#if genericObject.id != undefined}
		<IdDisplay id={genericObject.id} />
	{/if}
</h5>
{#if !short}
	<h6 class="card-subtitle text-body-secondary">{objectName}</h6>
{/if}
