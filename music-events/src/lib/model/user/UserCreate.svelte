<script lang="ts">
	import GenericObjectCreate from '$lib/generic_object_form/create/GenericObjectCreate.svelte';
	import { fieldComposer, type User } from './User';

	export let user: User;
	export let createCallback: (newUser: User) => Promise<number | undefined>;
</script>

<GenericObjectCreate createObject={user} objectName="User" {fieldComposer} {createCallback} />
