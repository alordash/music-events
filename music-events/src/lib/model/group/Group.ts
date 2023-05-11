import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { invoke } from "@tauri-apps/api/tauri";

export const GROUP_ID_LITERAL = 'group_id';

export type Group = {
    id: number,
    name: string,
    genre: string,
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'name':
            return FieldInfo('Name', FieldTypes.Text);
        case 'genre':
            return FieldInfo('Genre', FieldTypes.Text);
        default:
            return FieldInfoUnknown();
    }
}

export function createEmpty(): Group {
    return {
        id: 0,
        name: '',
        genre: ''
    }
}

export async function createGroup({ name, genre }: Group): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_group', { name, genre })
}

export async function getGroupsPaginated(count: number, offset: number): Promise<Array<Group>> {
    await sleepMaxOneSec();
    return await invoke('get_groups_paginated', { count, offset });
}

export async function getAllGroups(): Promise<Array<Group>> {
    await sleepMaxOneSec();
    return invoke('get_all_groups');
}

export async function getGroupsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_groups_count');
}

export async function getAllGroupIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_group_ids');
}

export async function getGroupById(groupId: number): Promise<Group | null> {
    await sleepMaxOneSec();
    return invoke('get_group_by_id', { groupId });
}

export async function addGroup(group: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_group', { group });
}

export async function updateGroup(group: Group): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_group', { group });
}

export async function removeGroup(groupId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_group', { groupId });
}