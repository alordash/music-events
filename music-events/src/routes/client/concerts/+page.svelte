<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { exploreComposer } from '$lib/generic_object_form/FieldInfo';
	import type { GenericObject } from '$lib/generic_object_form/GenericObject';
	import GenericObjectExplorer from '$lib/generic_object_form/explorer/GenericObjectExplorer.svelte';
	import { VIEWER_SEAT_ID_LITERAL } from '$lib/model/viewer_seat/ViewerSeat';
	import {
		CONCERT_ID_LITERAL,
		fieldComposer,
		getConcertsCount,
		getConcertsPaginated,
		getEventConcerts,
		type Concert
	} from '../../../lib/model/concert/Concert';

	function getExplorerAndCounter(): [
		(count: number, offset: number) => Promise<Array<Concert>>,
		() => Promise<number>
	] {
		const idStr = $page.url.searchParams.get(CONCERT_ID_LITERAL);
		if (idStr == null) {
			console.log(`Error getting artist id`);
			return [getConcertsPaginated, getConcertsCount];
		}
		const id = parseInt(idStr);
		console.log('event id :>> ', id);
		let baseFn = (count: number, offset: number) => {
			return getEventConcerts(id).then((concerts) => {
				return concerts.slice(offset, concerts.length).slice(0, count);
			});
		};
		let countFn = () => {
			return getEventConcerts(id).then((concerts) => concerts.length);
		};
		return [baseFn, countFn];
	}

	let [explorer, counter] = getExplorerAndCounter();

	function callback(concert: GenericObject | undefined) {
		goto(`/client/viewer_seats?${VIEWER_SEAT_ID_LITERAL}=${concert?.id}`);
	}
</script>

<GenericObjectExplorer
	short={true}
	showEditButton={false}
	clickCallback={callback}
	pageCapacity={12}
	columnsCount={4}
	{fieldComposer}
	editLiteral=""
	objectExplorer={exploreComposer(explorer)}
	objectName="concert"
	totalExplorer={counter}
/>
