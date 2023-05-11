<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import {
		fieldComposer,
		type Group,
		GROUP_ID_LITERAL
	} from './Group';

	export let group: Group;
</script>

<GenericObjectDisplay
	displayObject={group}
	objectName="Group"
	{fieldComposer}
	editLiteral={GROUP_ID_LITERAL}
/>
