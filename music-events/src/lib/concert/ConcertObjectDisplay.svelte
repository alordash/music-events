<script lang="ts">
	import ObjectDisplay from '$lib/generic_object_display/GenericObjectDisplay.svelte';
	import { fieldNameFormatter, fieldTypeExtractor, type Concert } from './Concert';

	export let concert: Concert;
</script>

<ObjectDisplay displayObject={concert} objectName='Concert' {fieldTypeExtractor} {fieldNameFormatter} />
