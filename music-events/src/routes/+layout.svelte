<script lang="ts">
	import LogoutButton from '$lib/authorization/LogoutButton.svelte';
	import { ACCOUNT_DEFAULT_ROLE } from '$lib/model/user/User';
	import { accountStore, getCurrentAccount, roleMapper } from '$lib/user_forms/AccountStore';

	let accountRole: string = getCurrentAccount().role;
	accountStore.subscribe(() => (accountRole = getCurrentAccount().role));
</script>

<div class="container w-75">
	<nav class="navbar navbar-expand-lg bg-body-tertiary">
		<div class="container-fluid">
			<a class="navbar-brand" href={roleMapper(accountRole)}>Home</a>
			<button
				class="navbar-toggler"
				type="button"
				data-bs-toggle="collapse"
				data-bs-target="#navbarNavAltMarkup"
				aria-controls="navbarNavAltMarkup"
				aria-expanded="false"
				aria-label="Toggle navigation"
			>
				<span class="navbar-toggler-icon" />
			</button>
			<div class="collapse navbar-collapse" id="navbarNavAltMarkup">
				<div class="navbar-nav">
					<a class="nav-link" href="/models/events/load/">Events</a>
					<a class="nav-link" href="/models/concerts/load/">Concerts</a>
					<a class="nav-link" href="/models/groups/load/">Groups</a>
					<a class="nav-link" href="/models/participants/load/">Participants</a>
					<a class="nav-link" href="/models/repertoires/load/">Repertoires</a>
					<a class="nav-link" href="/models/artists/load/">Artists</a>
					<a class="nav-link" href="/models/group_artists/load/">Group artists</a>
					<a class="nav-link" href="/models/viewer_seats/load/">Viewer seats</a>
					<a class="nav-link" href="/models/viewers/load/">Viewers</a>
					<a class="nav-link" href="/models/persons/load/">Persons</a>
					<a class="nav-link" href="/models/users/load/">Users</a>
					{#if accountRole != ACCOUNT_DEFAULT_ROLE}
						<LogoutButton />
					{/if}
				</div>
			</div>
		</div>
	</nav>

	<slot />
</div>
