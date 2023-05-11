import { sleepMaxOneSec } from '$lib/Timer';
import { FieldTypes } from '$lib/generic_object_form/FieldTypes';
import { FieldInfo, FieldInfoUnknown, exploreComposer } from '$lib/generic_object_form/FieldInfo';
import { invoke } from '@tauri-apps/api/tauri';
import { Decimal } from 'decimal.js'
import { getConcertById, getConcertsCount, getConcertsPaginated } from '../concert/Concert';
import type { GenericObject } from '$lib/generic_object_form/GenericObject';

export const VIEWER_SEAT_ID_LITERAL = 'viewer_seat_id';

export type ViewerSeat = {
    id: number;
    kind: string;
    costRubles: Decimal;
    realNumber: number;
    concertId: number;
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'kind':
            return FieldInfo('Type', FieldTypes.Text);
        case 'costRubles':
            return FieldInfo('Cost', FieldTypes.CostRubles);
        case 'realNumber':
            return FieldInfo('Number', FieldTypes.Number);
        case 'concertId':
            return FieldInfo(
                'Concert',
                FieldTypes.ObjectReference,
                100000,
                getConcertById,
                exploreComposer(getConcertsPaginated),
                getConcertsCount,
                fieldComposer,
                "concerts"
            );
        default:
            return FieldInfoUnknown();
    }
}

export async function nameComposer(obj: GenericObject) {
    const viewerSeat = <ViewerSeat>obj;
    const concert = await getConcertById(viewerSeat.concertId);
    let result = `${concert?.name}: ${viewerSeat.kind}`;
    if (viewerSeat.realNumber != 0) {
        result += ` №${viewerSeat.realNumber}`;
    }
    return result;
}

export function createEmpty(): ViewerSeat {
    return {
        id: 0,
        concertId: 0,
        costRubles: new Decimal(0),
        kind: '',
        realNumber: 0
    }
}

export async function createViewerSeat({ kind, costRubles, realNumber, concertId }: ViewerSeat): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_viewer_seat', { kind, costRubles, realNumber, concertId });
}

export async function getViewerSeatsPaginated(count: number, offset: number): Promise<Array<ViewerSeat>> {
    await sleepMaxOneSec();
    return await invoke('get_viewer_seats_paginated', { count, offset });
}

export async function getAllViewerSeats(): Promise<Array<ViewerSeat>> {
    await sleepMaxOneSec();
    return invoke('get_all_viewer_seats');
}

export async function getViewerSeatsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_viewer_seats_count');
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