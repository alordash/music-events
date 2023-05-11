<script lang="ts">
	import type { NameComposer } from '$lib/generic_object_form/FieldInfo';
	import type { GenericObject } from '$lib/generic_object_form/GenericObject';

	export let name: string | undefined;
	export let genericObject: GenericObject | undefined = undefined;
	export let nameComposer: NameComposer | undefined = undefined;
	export let light = false;

	function getComposedNamePromise() {
		let unknown = Promise.resolve('???');
		if (genericObject == undefined) {
			return unknown;
		}
		if (nameComposer == undefined) {
			return unknown;
		}
		return nameComposer(genericObject);
	}
</script>

<div class={light ? '' : 'fw-bold'}>
	{#if nameComposer != undefined}
		{#await getComposedNamePromise()}
			<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
		{:then composedName}
			{composedName}
		{/await}
	{:else if name != undefined}
		{name}
	{/if}
</div>
