import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { invoke } from "@tauri-apps/api/tauri";
import type { Artist } from "../artist/Artist";

export const GROUP_ID_LITERAL = 'group_id';

export type Group = {
    id: number,
    name: string,
    genre: string,
    artists: Array<Artist>
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'name':
            return FieldInfo('Name', FieldTypes.Name);
        case 'genre':
            return FieldInfo('Genre', FieldTypes.Text);
        case 'artists':
            return FieldInfo('Artists', FieldTypes.GroupArtistsAggregated, 10000);
        default:
            return FieldInfoUnknown();
    }
}

export function createEmpty(): Group {
    return {
        id: 0,
        name: '',
        genre: '',
        artists: []
    }
}

function addGroupArtists(objUnknown: unknown) {
    const obj = <object>objUnknown;
    return { artists: [], ...obj }
}

function arrayAddGroupArtists(arrUnknown: unknown) {
    const arr = <Array<object>>arrUnknown;
    return <Array<Group>>(<Array<unknown>>arr.map(addGroupArtists));
}

export async function createGroup({ name, genre }: Group): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_group', { name, genre }).then(addGroupArtists);
}

export async function getGroupsPaginated(count: number, offset: number): Promise<Array<Group>> {
    await sleepMaxOneSec();
    return await invoke('get_groups_paginated', { count, offset }).then(arrayAddGroupArtists);
}

export async function getAllGroups(): Promise<Array<Group>> {
    await sleepMaxOneSec();
    return invoke('get_all_groups').then(arrayAddGroupArtists);
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

export async function getConcertGroups(concertId: number): Promise<Array<Group>> {
    await sleepMaxOneSec();
    return invoke('get_concert_groups', { concertId });
}