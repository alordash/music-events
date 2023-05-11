import { sleepMaxOneSec } from '$lib/Timer';
import { FieldTypes } from '$lib/generic_object_form/FieldTypes';
import { FieldInfo, FieldInfoUnknown, exploreComposer } from '$lib/generic_object_form/FieldInfo';
import { invoke } from '@tauri-apps/api/tauri';
import type { GenericObject } from '$lib/generic_object_form/GenericObject';
import { getPersonById, getPersonsCount, getPersonsPaginated } from '../person/Person';

export const ACTOR_ID_LITERAL = 'actor_id';

export type Actor = {
    id: number;
    pseudonym: string,
    personId: number
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'pseudonym':
            return FieldInfo('Pseudonym', FieldTypes.Text);
        case 'personId':
            return FieldInfo(
                'Person',
                FieldTypes.ObjectReference,
                10000,
                getPersonById,
                exploreComposer(getPersonsPaginated),
                getPersonsCount,
                fieldComposer,
                "persons"
            );
        default:
            return FieldInfoUnknown();
    }
}

export async function nameComposer(obj: GenericObject) {
    const actor = <Actor>obj;
    return Promise.resolve(actor.pseudonym);
}

export function createEmpty(): Actor {
    return {
        id: 0,
        pseudonym: '',
        personId: 0
    }
}

export async function createActor({ pseudonym, personId }: Actor): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_actor', { pseudonym, personId });
}

export async function getActorsPaginated(count: number, offset: number): Promise<Array<Actor>> {
    await sleepMaxOneSec();
    return await invoke('get_actors_paginated', { count, offset });
}

export async function getAllActors(): Promise<Array<Actor>> {
    await sleepMaxOneSec();
    return invoke('get_all_actors');
}

export async function getActorsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_actors_count');
}

export async function getAllActorIdsAndRealNumbersAndConcertNames(): Promise<Array<[number, number, string]>> {
    await sleepMaxOneSec();
    return invoke('get_all_actor_ids_and_real_numbers_and_concert_names');
}

export async function getActorById(actorId: number): Promise<Actor | null> {
    await sleepMaxOneSec();
    return invoke('get_actor_by_id', { actorId });
}

export async function addActor(actor: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_actor', { actor })
}

export async function updateActor(actor: Actor): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_actor', { actor });
}

export async function removeActor(actorId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_actor', { actorId });
}