import { sleepMaxOneSec } from "$lib/Timer";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { FieldInfo } from "$lib/generic_object_form/edit/FieldInfo";
import { invoke } from "@tauri-apps/api/tauri";

export const CONCERT_ID_LITERAL = 'concert_id';

export type Concert = {
    id: number;
    date: string;
    durationMinutes: number;
    address: string;
    name: string;
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'date':
            return FieldInfo('Date', FieldTypes.Date);
        case 'durationMinutes':
            return FieldInfo('Duration', FieldTypes.DurationMinutes);
        case 'address':
            return FieldInfo('Address', FieldTypes.Text);
        case 'name':
            return FieldInfo('Name', FieldTypes.Text);
        default:
            return FieldInfo('', FieldTypes.Text);
    }
}

export function createEmptyConcert(): Concert {
    return {
        id: 0,
        address: '',
        date: '',
        durationMinutes: 0,
        name: ''
    };
}

export async function createConcert({ date, durationMinutes, address, name }: Concert): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_concert', { date, durationMinutes, address, name })
}

export async function getConcertsPaginated(count: number, offset: number): Promise<Array<Concert>> {
    await sleepMaxOneSec();
    return await invoke('get_concerts_paginated', { count, offset });
}

export async function getAllConcerts(): Promise<Array<Concert>> {
    await sleepMaxOneSec();
    return invoke('get_all_concerts');
}

export async function getConcertsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_concerts_count');
}

export async function getAllConcertIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_concert_ids');
}

export async function getAllConcertIdsAndNames(): Promise<Array<[number, string]>> {
    await sleepMaxOneSec();
    return invoke('get_all_concert_ids_and_names');
}

export async function getConcertById(concertId: number): Promise<Concert | null> {
    await sleepMaxOneSec();
    return invoke('get_concert_by_id', { concertId });
}

export async function addConcert(concert: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_concert', { concert });
}

export async function updateConcert(concert: Concert): Promise<void> {
    await sleepMaxOneSec();
    console.log('update concert :>> ', concert);
    return invoke('update_concert', { concert });
}

export async function removeConcert(concertId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_concert', { concertId });
}