import { sleepMaxOneSec } from "$lib/Timer";
import { invoke } from "@tauri-apps/api/tauri";

export const USER_PERSON_ID_LITERAL = 'user_person_id';

export type UserPerson = {
    id: number,
    userId: number,
    personId: number
}

export function createEmpty(): UserPerson {
    return {
        id: 0,
        userId: 0,
        personId: 0
    }
}

export async function createUserPerson({ userId, personId }: UserPerson): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_user_person', { userId, personId })
}

export async function getUserPersonsPaginated(count: number, offset: number): Promise<Array<UserPerson>> {
    await sleepMaxOneSec();
    return await invoke('get_user_persons_paginated', { count, offset });
}

export async function getAllUserPersons(): Promise<Array<UserPerson>> {
    await sleepMaxOneSec();
    return invoke('get_all_user_persons');
}

export async function getUserPersonsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_user_persons_count');
}

export async function getAllUserPersonIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_user_person_ids');
}

export async function getUserPersonById(userPersonId: number): Promise<UserPerson | null> {
    await sleepMaxOneSec();
    return invoke('get_user_person_by_id', { userPersonId });
}

export async function addUserPerson(userPerson: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_user_person', { userPerson });
}

export async function updateUserPerson(userPerson: UserPerson): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_user_person', { userPerson });
}

export async function removeUserPerson(userPersonId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_user_person', { user_personId: userPersonId });
}

export async function getUserPersonByUserIdAndPersonId(userId: number, personId: number): Promise<UserPerson | null> {
    await sleepMaxOneSec();
    return invoke('get_user_person_by_user_id_and_person_id', { userId, personId });
}

export async function removeUserPersonByPersonId(personId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_user_person_by_person_id', { personId });
}