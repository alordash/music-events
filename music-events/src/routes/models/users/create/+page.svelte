<script lang="ts">
	import { createEmpty, addUser } from '$lib/model/user/User';
	import UserObjectCreate from '$lib/model/user/UserCreate.svelte';
	let user = createEmpty();
</script>

<div class="w-50">
	<UserObjectCreate {user} createCallback={addUser} />
</div>
