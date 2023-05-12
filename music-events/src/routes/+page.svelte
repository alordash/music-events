<script lang="ts">
	import { goto } from '$app/navigation';
	import LoginForm from '$lib/authorization/LoginForm.svelte';
	import { getCurrentAccount, roleMapper } from '$lib/user_forms/AccountStore';
	let account = getCurrentAccount();
	let redirect = roleMapper(account.role);
	if (redirect != '/') {
		goto(redirect);
	}
</script>

<LoginForm />
