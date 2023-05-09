<script>
	import { addViewerSeat, createEmpty } from '$lib/model/viewer_seat/ViewerSeat';
	import ViewerSeatCreate from '$lib/model/viewer_seat/ViewerSeatCreate.svelte';
	let viewerSeat = createEmpty();
</script>

<ViewerSeatCreate {viewerSeat} createCallback={addViewerSeat} />
