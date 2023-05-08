import { sleepMaxOneSec } from '$lib/Timer';
import { FieldTypes } from '$lib/generic_object_form/FieldTypes';
import { FieldInfo } from '$lib/generic_object_form/edit/FieldInfo';
import { invoke } from '@tauri-apps/api/tauri';
import type { Decimal } from 'decimal.js'

export type ViewerSeat = {
    id: number;
    kind: string;
    costRubles: Decimal;
    realNumber: number;
    concertId: number;
}

export function fieldTypeExtractor(fieldName: string): FieldTypes {
    switch (fieldName) {
        case 'id':
            return FieldTypes.Id;
        case 'kind':
            return FieldTypes.Text;
        case 'costRubles':
            return FieldTypes.Number
        case 'realNumber':
            return FieldTypes.Text;
        case 'concertId':
            return FieldTypes.ObjectSelector;
        default:
            return FieldTypes.Text;
    }
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'kind':
            return FieldInfo('Type', FieldTypes.Text);
        case 'costRubles':
            return FieldInfo('Cost ₽', FieldTypes.Number);
        case 'realNumber':
            return FieldInfo('Number', FieldTypes.Text);
        case 'concertId':
            return FieldInfo('Concert', FieldTypes.ObjectSelector);
        default:
            return FieldInfo('', FieldTypes.Text);
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