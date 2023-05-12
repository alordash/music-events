<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { exploreComposer } from '$lib/generic_object_form/FieldInfo';
	import type { GenericObject } from '$lib/generic_object_form/GenericObject';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import {
		VIEWER_SEAT_ID_LITERAL,
		fieldComposer,
		getViewerSeatsCount,
		getViewerSeatsPaginated,
		type ViewerSeat,
		getConcertViewerSeats,
		nameComposer,
		getFreeViewerSeatsPaginated,
		getFreeViewerSeatsCount,
		getFreeConcertViewerSeats
	} from '../../../lib/model/viewer_seat/ViewerSeat';

	function getExplorerAndCounter(): [
		(count: number, offset: number) => Promise<Array<ViewerSeat>>,
		() => Promise<number>
	] {
		const idStr = $page.url.searchParams.get(VIEWER_SEAT_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting artist id`);
			return [getFreeViewerSeatsPaginated, getFreeViewerSeatsCount];
		}
		const id = parseInt(idStr);
		console.log('event id :>> ', id);
		let baseFn = (count: number, offset: number) => {
			return getFreeConcertViewerSeats(id).then((viewerSeats) => {
				return viewerSeats.slice(offset, viewerSeats.length).slice(0, count);
			});
		};
		let countFn = () => {
			return getFreeConcertViewerSeats(id).then((viewerSeats) => viewerSeats.length);
		};
		return [baseFn, countFn];
	}

	let [explorer, counter] = getExplorerAndCounter();

	function callback(viewerSeat: GenericObject | undefined) {
		goto(`/client/viewer_seat?${VIEWER_SEAT_ID_LITERAL}=${viewerSeat?.id}`);
	}
</script>

<GenericObjectExplorer
	short={false}
	showEditButton={false}
	clickCallback={callback}
	pageCapacity={12}
	columnsCount={4}
	{fieldComposer}
	editLiteral=""
	objectExplorer={exploreComposer(explorer)}
	objectName="viewer seat"
	totalExplorer={counter}
	{nameComposer}
/>
