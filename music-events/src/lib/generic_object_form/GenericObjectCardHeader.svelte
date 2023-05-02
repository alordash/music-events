<script lang="ts">
	import type { GenericObject } from './GenericObject';
	import IdDisplay from './display/field_displays/IdDisplay.svelte';
	import NameDisplay from './display/field_displays/NameDisplay.svelte';

	export let genericObject: GenericObject;
	export let objectName: string;
</script>

<h5 class="card-title user-select-all">
	{#if genericObject.name != undefined}
		<NameDisplay name={genericObject.name} />
	{/if}
	{#if genericObject.id != undefined}
		<IdDisplay id={genericObject.id} />
	{/if}
</h5>
<h6 class="card-subtitle text-body-secondary">{objectName}</h6>
