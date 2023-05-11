import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { invoke } from "@tauri-apps/api/tauri";

export const EVENT_ID_LITERAL = 'event_id';

export type Event = {
    id: number,
    name: string
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'name':
            return FieldInfo('Name', FieldTypes.Text)

        default:
            return FieldInfoUnknown();
    }
}

export function createEmpty(): Event {
    return {
        id: 0,
        name: ''
    }
}

export async function createEvent({ name }: Event): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_event', { name })
}

export async function getEventsPaginated(count: number, offset: number): Promise<Array<Event>> {
    await sleepMaxOneSec();
    return await invoke('get_events_paginated', { count, offset });
}

export async function getAllEvents(): Promise<Array<Event>> {
    await sleepMaxOneSec();
    return invoke('get_all_events');
}

export async function getEventsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_events_count');
}

export async function getAllEventIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_event_ids');
}

export async function getEventById(eventId: number): Promise<Event | null> {
    await sleepMaxOneSec();
    return invoke('get_event_by_id', { eventId });
}

export async function addEvent(event: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_event', { event });
}

export async function updateEvent(event: Event): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_event', { event });
}

export async function removeEvent(eventId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_event', { eventId });
}