<script lang="ts">
	import { createEmpty, addArtist } from '$lib/model/artist/Artist';
	import ArtistObjectCreate from '$lib/model/artist/ArtistCreate.svelte';
	let artist = createEmpty();
</script>

<div class="w-50">
	<ArtistObjectCreate {artist} createCallback={addArtist} />
</div>
