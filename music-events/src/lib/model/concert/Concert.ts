import { sleepMaxOneSec } from "$lib/Timer";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { FieldInfo, FieldInfoUnknown, exploreComposer } from "$lib/generic_object_form/FieldInfo";
import { invoke } from "@tauri-apps/api/tauri";
import { getEventById, getEventsCount, getEventsPaginated } from "../event/Event";
import type { Group } from "../group/Group";

export const CONCERT_ID_LITERAL = 'concert_id';

export type Concert = {
    id: number;
    date: string;
    durationMinutes: number;
    address: string;
    name: string;
    eventId: number;
    groups: Array<Group>
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
            return FieldInfo('Name', FieldTypes.Name);
        case 'eventId':
            return FieldInfo(
                'Event',
                FieldTypes.ObjectReference,
                10000,
                getEventById,
                exploreComposer(getEventsPaginated),
                getEventsCount,
                fieldComposer,
                "events"
            );
        case 'groups':
            return FieldInfo('Groups', FieldTypes.ConcertGroupsAggregated, 20000)
        default:
            return FieldInfoUnknown();
    }
}

export function createEmpty(): Concert {
    return {
        id: 0,
        address: '',
        date: '',
        durationMinutes: 0,
        name: '',
        eventId: 0,
        groups: []
    };
}

function addConcertGroups(objUnknown: unknown) {
    const obj = <object>objUnknown;
    return { groups: [], ...obj }
}

function arrayAddConcertGroups(arrUnknown: unknown) {
    const arr = <Array<object>>arrUnknown;
    return <Array<Concert>>(<Array<unknown>>arr.map(addConcertGroups));
}

export async function createConcert({ date, durationMinutes, address, name }: Concert): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_concert', { date, durationMinutes, address, name }).then(addConcertGroups);
}

export async function getConcertsPaginated(count: number, offset: number): Promise<Array<Concert>> {
    await sleepMaxOneSec();
    return await invoke('get_concerts_paginated', { count, offset }).then(arrayAddConcertGroups);
}

export async function getAllConcerts(): Promise<Array<Concert>> {
    await sleepMaxOneSec();
    return invoke('get_all_concerts').then(arrayAddConcertGroups);
}

export async function getConcertsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_concerts_count');
}

export async function getAllConcertIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_concert_ids');
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
    return invoke('update_concert', { concert });
}

export async function removeConcert(concertId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_concert', { concertId });
}

export async function getEventConcerts(eventId: number): Promise<Array<Concert>> {
    await sleepMaxOneSec();
    return invoke('get_event_concerts', { eventId });
}