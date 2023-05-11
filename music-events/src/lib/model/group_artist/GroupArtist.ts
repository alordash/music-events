import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown, exploreComposer } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { invoke } from "@tauri-apps/api/tauri";
import { getGroupById, getGroupsCount, getGroupsPaginated } from "../group/Group";
import { getArtistById, getArtistsCount, getArtistsPaginated } from '../artist/Artist';
import { nameComposer as artistNameComposer } from '../artist/Artist'

export const GROUP_ARTIST_ID_LITERAL = 'group_artist_id';

export type GroupArtist = {
    id: number,
    groupId: number,
    artistId: number,
    role: string
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'groupId':
            return FieldInfo(
                'Group',
                FieldTypes.ObjectReference,
                20000,
                getGroupById,
                exploreComposer(getGroupsPaginated),
                getGroupsCount,
                fieldComposer,
                "groups"
            );
        case 'artistId':
            return FieldInfo(
                'Artist',
                FieldTypes.ObjectReference,
                10000,
                getArtistById,
                exploreComposer(getArtistsPaginated),
                getArtistsCount,
                fieldComposer,
                "artists",
                artistNameComposer
            );
        case 'role':
            return FieldInfo(
                'Role',
                FieldTypes.Text,
                30000
            );

        default:
            return FieldInfoUnknown();
    }
}

export function createEmpty(): GroupArtist {
    return {
        id: 0,
        groupId: 0,
        artistId: 0,
        role: ''
    }
}

export async function createGroupArtist({ groupId, artistId, role }: GroupArtist): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_group_artist', { groupId, artistId, role })
}

export async function getGroupArtistsPaginated(count: number, offset: number): Promise<Array<GroupArtist>> {
    await sleepMaxOneSec();
    return await invoke('get_group_artists_paginated', { count, offset });
}

export async function getAllGroupArtists(): Promise<Array<GroupArtist>> {
    await sleepMaxOneSec();
    return invoke('get_all_group_artists');
}

export async function getGroupArtistsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_group_artists_count');
}

export async function getAllGroupArtistIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_group_artist_ids');
}

export async function getGroupArtistById(groupArtistId: number): Promise<GroupArtist | null> {
    await sleepMaxOneSec();
    return invoke('get_group_artist_by_id', { groupArtistId });
}

export async function addGroupArtist(groupArtist: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_group_artist', { groupArtist });
}

export async function updateGroupArtist(groupArtist: GroupArtist): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_group_artist', { groupArtist });
}

export async function removeGroupArtist(group_artistId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_group_artist', { group_artistId });
}