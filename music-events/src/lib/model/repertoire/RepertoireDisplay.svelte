<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import { fieldComposer, type Repertoire, REPERTOIRE_ID_LITERAL } from './Repertoire';

	export let repertoire: Repertoire;
</script>

<GenericObjectDisplay
	displayObject={repertoire}
	objectName="Repertoire"
	{fieldComposer}
	editLiteral={REPERTOIRE_ID_LITERAL}
/>
