<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import { fieldComposer, type Actor, ACTOR_ID_LITERAL, nameComposer } from './Actor';

	export let actor: Actor;
</script>

<GenericObjectDisplay
	displayObject={actor}
	objectName="Actor seat"
	{fieldComposer}
	editLiteral={ACTOR_ID_LITERAL}
	{nameComposer}
/>
