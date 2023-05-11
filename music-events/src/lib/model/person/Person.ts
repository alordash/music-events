import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import type { GenericObject } from "$lib/generic_object_form/GenericObject";
import { invoke } from "@tauri-apps/api/tauri";

export const PERSON_ID_LITERAL = 'person_id';

export type Person = {
    id: number,
    name: string,
    surname: string
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'name':
            return FieldInfo('Name', FieldTypes.Text)
        case 'surname':
            return FieldInfo('Surname', FieldTypes.Text)

        default:
            return FieldInfoUnknown();
    }
}

export async function nameComposer(obj: GenericObject) {
    const person = <Person>obj;
    return Promise.resolve(`${person.name} ${person.surname}`);
}

export function createEmpty(): Person {
    return {
        id: 0,
        name: '',
        surname: '',
    }
}

export async function createPerson({ name }: Person): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_person', { name })
}

export async function getPersonsPaginated(count: number, offset: number): Promise<Array<Person>> {
    await sleepMaxOneSec();
    return await invoke('get_persons_paginated', { count, offset });
}

export async function getAllPersons(): Promise<Array<Person>> {
    await sleepMaxOneSec();
    return invoke('get_all_persons');
}

export async function getPersonsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_persons_count');
}

export async function getAllPersonIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_person_ids');
}

export async function getPersonById(personId: number): Promise<Person | null> {
    await sleepMaxOneSec();
    return invoke('get_person_by_id', { personId });
}

export async function addPerson(person: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_person', { person });
}

export async function updatePerson(person: Person): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_person', { person });
}

export async function removePerson(personId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_person', { personId });
}