<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import { fieldNameFormatter, fieldTypeExtractor, type Concert } from './Concert';

	export let concert: Concert;
</script>

<GenericObjectDisplay displayObject={concert} objectName='Concert' {fieldTypeExtractor} {fieldNameFormatter} />
