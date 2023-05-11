<script lang="ts">
	import GenericObjectEdit from '$lib/generic_object_form/edit/GenericObjectEdit.svelte';
	import { fieldComposer, type User } from './User';

	export let user: User;
	export let changeCallback: (newUser: User) => void;
	export let deleteCallback: (deleteUser: User) => void;
</script>

<GenericObjectEdit
	editObject={user}
	objectName="User"
	{fieldComposer}
	{changeCallback}
	{deleteCallback}
/>
