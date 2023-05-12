<script lang="ts">
	import { goto } from '$app/navigation';
	import { ACCOUNT_DEFAULT_ROLE, createEmpty } from '$lib/model/user/User';
	import { accountStore, roleMapper as roleRedirectMapper, setCurrentAccount } from '$lib/user_forms/AccountStore';

	function onLogoutClick() {
		setCurrentAccount(createEmpty());
		// accountStore.set(createEmpty());
		let redirect = roleRedirectMapper(ACCOUNT_DEFAULT_ROLE);
		goto(redirect);
	}
</script>

<button class="btn btn-danger" on:click={onLogoutClick}>Log out</button>
