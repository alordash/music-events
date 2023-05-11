<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type Actor } from './Actor';

	export let actor: Actor;
	export let changeCallback: (newActor: Actor) => void;
	export let deleteCallback: (deleteActor: Actor) => void;
</script>

<GenericObjectEdit
	editObject={actor}
	objectName="Actor seat"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
