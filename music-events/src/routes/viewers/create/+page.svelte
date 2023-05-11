<script lang="ts">
	import { createEmpty, addViewer } from '$lib/model/viewer/Viewer';
	import ViewerObjectCreate from '$lib/model/viewer/ViewerCreate.svelte';
	let viewer = createEmpty();
</script>

<div class="w-50">
	<ViewerObjectCreate {viewer} createCallback={addViewer} />
</div>
