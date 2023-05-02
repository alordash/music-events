<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import {
		fieldNameFormatter,
		fieldTypeExtractor,
		type Concert,
		CONCERT_ID_LITERAL
	} from './Concert';

	export let concert: Concert;
</script>

<GenericObjectDisplay
	displayObject={concert}
	objectName="Concert"
	{fieldTypeExtractor}
	{fieldNameFormatter}
	editLiteral={CONCERT_ID_LITERAL}
/>
