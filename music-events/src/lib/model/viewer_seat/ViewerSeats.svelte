<script lang="ts">
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import { getViewersCount } from '../viewer/Viewer';
	import {
		fieldComposer,
		VIEWER_SEAT_ID_LITERAL,
		getViewerSeatsPaginated,
		nameComposer,
		getAllViewerSeats
	} from './ViewerSeat';

	function objectExplorer(count: number, offset: number) {
		return getViewerSeatsPaginated(count, offset).then((objects) =>
			Promise.resolve({ objects, offset })
		);
	}
</script>

<GenericObjectExplorer
	globalObjectExplorer={getAllViewerSeats}
	{objectExplorer}
	totalExplorer={getViewersCount}
	objectName="Viewer seat"
	{fieldComposer}
	editLiteral={VIEWER_SEAT_ID_LITERAL}
	pageCapacity={3}
	{nameComposer}
/>
