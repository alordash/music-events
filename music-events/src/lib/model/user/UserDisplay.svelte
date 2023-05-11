<script lang="ts">
	import GenericObjectDisplay from '$lib/generic_object_form/display/GenericObjectDisplay.svelte';
	import { nameComposer } from '../user/User';
	import { fieldComposer, type User, USER_ID_LITERAL } from './User';

	export let user: User;
</script>

<GenericObjectDisplay
	displayObject={user}
	objectName="User"
	{fieldComposer}
	editLiteral={USER_ID_LITERAL}
	{nameComposer}
/>
