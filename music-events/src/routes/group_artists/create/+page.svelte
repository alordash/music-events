<script>
	import { addGroupArtist, createEmpty } from '$lib/model/group_artist/GroupArtist';
	import GroupArtistCreate from '$lib/model/group_artist/GroupArtistCreate.svelte';
	let groupArtist = createEmpty();
</script>

<GroupArtistCreate {groupArtist} createCallback={addGroupArtist} />
