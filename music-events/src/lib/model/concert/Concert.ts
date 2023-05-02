import { sleepMaxOneSec } from "$lib/Timer";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { invoke } from "@tauri-apps/api/tauri";

export const CONCERT_ID_LITERAL = 'concert_id';

export type Concert = {
    id: number;
    date: string;
    durationMinutes: number;
    address: string;
    name: string;
}

export function fieldTypeExtractor(fieldName: string): FieldTypes {
    switch (fieldName) {
        case 'id':
            return FieldTypes.Id;
        case 'durationMinutes':
            return FieldTypes.DurationMinutes;
        case 'date':
            return FieldTypes.Date;

        default:
            return FieldTypes.Text
    }
}

export function fieldNameFormatter(fieldName: string): string {
    switch (fieldName) {
        case 'id':
            return 'Id';
        case 'date':
            return 'Date';
        case 'durationMinutes':
            return 'Duration';
        case 'address':
            return 'Address';
        case 'name':
            return 'Name';
        default:
            return '';
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

export async function getAllConcerts(): Promise<Array<Concert>> {
    await sleepMaxOneSec();
    return invoke('get_all_concerts');
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