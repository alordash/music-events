import { sleepMaxOneSec } from '$lib/Timer';
import { invoke } from '@tauri-apps/api/tauri';
import type { Decimal } from 'decimal.js'

export class ViewerSeat {
    id: number;
    kind: string;
    costRubles: Decimal;
    realNumber: number;
    concertId: number;

    constructor(id: number, kind: string, costRubles: Decimal, realNumber: number, concertId: number) {
        this.id = id;
        this.kind = kind;
        this.costRubles = costRubles;
        this.realNumber = realNumber;
        this.concertId = concertId;
    }
}

export async function createViewerSeat({ kind, costRubles, realNumber, concertId }: ViewerSeat): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_viewer_seat', { kind, costRubles, realNumber, concertId });
}

export async function getNViewerSeatsWithOffset(count: number, offset: number): Promise<Array<ViewerSeat>> {
    await sleepMaxOneSec();
    return await invoke('get_n_viewer_seats_with_offset', { count, offset });
}

export async function getAllViewerSeats(): Promise<Array<ViewerSeat>> {
    await sleepMaxOneSec();
    return invoke('get_all_viewer_seats');
}

export async function getAllViewerSeatIdsAndRealNumbersAndConcertNames(): Promise<Array<[number, number, string]>> {
    await sleepMaxOneSec();
    return invoke('get_all_viewer_seat_ids_and_real_numbers_and_concert_names');
}

export async function getViewerSeatById(viewerSeatId: number): Promise<ViewerSeat | null> {
    await sleepMaxOneSec();
    return invoke('get_viewer_seat_by_id', { viewerSeatId });
}

export async function addViewerSeat(viewerSeat: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_viewer_seat', { viewerSeat })
}

export async function updateViewerSeat(viewerSeat: ViewerSeat): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_viewer_seat', { viewerSeat });
}

export async function removeViewerSeat(viewerSeatId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_viewer_seat', { viewerSeatId });
}