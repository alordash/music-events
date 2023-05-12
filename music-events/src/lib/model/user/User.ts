import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import type { GenericObject } from "$lib/generic_object_form/GenericObject";
import { invoke } from "@tauri-apps/api/tauri";
import type { Person } from "../person/Person";
import type { ViewerSeat } from "../viewer_seat/ViewerSeat";
import type { Viewer } from "../viewer/Viewer";

export const USER_ID_LITERAL = 'user_id';
export const ACCOUNT_DEFAULT_ROLE = 'guest';

export type User = {
    id: number,
    login: string,
    password: string,
    role: string,
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'login':
            return FieldInfo('Login', FieldTypes.Text);
        case 'password':
            return FieldInfo('Password', FieldTypes.Password);
        case 'role':
            return FieldInfo('Role', FieldTypes.Text);

        default:
            return FieldInfoUnknown();
    }
}

export async function nameComposer(obj: GenericObject) {
    const user = <User>obj;
    return user.login;
}

export function createEmpty(): User {
    return {
        id: 0,
        login: '',
        password: '',
        role: ACCOUNT_DEFAULT_ROLE
    }
}

export async function createUser({ login, password, role }: User): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_user', { login, password, role })
}

export async function getUsersPaginated(count: number, offset: number): Promise<Array<User>> {
    await sleepMaxOneSec();
    return await invoke('get_users_paginated', { count, offset });
}

export async function getAllUsers(): Promise<Array<User>> {
    await sleepMaxOneSec();
    return invoke('get_all_users');
}

export async function getUsersCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_users_count');
}

export async function getAllUserIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_user_ids');
}

export async function getUserById(userId: number): Promise<User | null> {
    await sleepMaxOneSec();
    return invoke('get_user_by_id', { userId });
}

export async function addUser(user: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_user', { user });
}

export async function updateUser(user: User): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_user', { user });
}

export async function removeUser(userId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_user', { userId });
}

export async function tryLogin(login: string, password: string): Promise<object> {
    await sleepMaxOneSec();
    return invoke('try_login_user', { login, password })
}

export async function getBoughtViewerSeats(userId: number): Promise<Array<[Person, Viewer, ViewerSeat]>> {
    await sleepMaxOneSec();
    return invoke('get_bought_viewer_seats', { userId });
}