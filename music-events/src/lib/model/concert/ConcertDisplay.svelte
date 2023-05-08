<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import {
		fieldComposer,
		type Concert,
		CONCERT_ID_LITERAL
	} from './Concert';

	export let concert: Concert;
</script>

<GenericObjectDisplay
	displayObject={concert}
	objectName="Concert"
	{fieldComposer}
	editLiteral={CONCERT_ID_LITERAL}
/>
