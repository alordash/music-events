<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		ACCOUNT_DEFAULT_ROLE,
		accountRoleStore,
		roleMapper as roleRedirectMapper
	} from '$lib/user_forms/AccountStore';

	function onLogoutClick() {
		accountRoleStore.set(ACCOUNT_DEFAULT_ROLE);
		let redirect = roleRedirectMapper(ACCOUNT_DEFAULT_ROLE);
		goto(redirect);
	}
</script>

<button class="btn btn-danger" on:click={onLogoutClick}>Log out</button>
