<script lang="ts">
	import { createEmpty, addGroup } from '$lib/model/group/Group';
	import GroupObjectCreate from '$lib/model/group/GroupCreate.svelte';
	let group = createEmpty();
</script>

<div class="w-50">
	<GroupObjectCreate {group} createCallback={addGroup} />
</div>
