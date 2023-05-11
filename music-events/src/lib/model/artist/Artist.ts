import { sleepMaxOneSec } from '$lib/Timer';
import { FieldTypes } from '$lib/generic_object_form/FieldTypes';
import { FieldInfo, FieldInfoUnknown, exploreComposer } from '$lib/generic_object_form/FieldInfo';
import { invoke } from '@tauri-apps/api/tauri';
import type { GenericObject } from '$lib/generic_object_form/GenericObject';
import { getPersonById, getPersonsCount, getPersonsPaginated } from '../person/Person';
import { nameComposer as personNameComposer } from '../person/Person'

export const ARTIST_ID_LITERAL = 'artist_id';

export type Artist = {
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
                "persons",
                personNameComposer
            );
        default:
            return FieldInfoUnknown();
    }
}

export async function nameComposer(obj: GenericObject) {
    const artist = <Artist>obj;
    return Promise.resolve(artist.pseudonym);
}

export function createEmpty(): Artist {
    return {
        id: 0,
        pseudonym: '',
        personId: 0
    }
}

export async function createArtist({ pseudonym, personId }: Artist): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_artist', { pseudonym, personId });
}

export async function getArtistsPaginated(count: number, offset: number): Promise<Array<Artist>> {
    await sleepMaxOneSec();
    return await invoke('get_artists_paginated', { count, offset });
}

export async function getAllArtists(): Promise<Array<Artist>> {
    await sleepMaxOneSec();
    return invoke('get_all_artists');
}

export async function getArtistsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_artists_count');
}

export async function getAllArtistIdsAndRealNumbersAndConcertNames(): Promise<Array<[number, number, string]>> {
    await sleepMaxOneSec();
    return invoke('get_all_artist_ids_and_real_numbers_and_concert_names');
}

export async function getArtistById(artistId: number): Promise<Artist | null> {
    await sleepMaxOneSec();
    return invoke('get_artist_by_id', { artistId });
}

export async function addArtist(artist: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_artist', { artist })
}

export async function updateArtist(artist: Artist): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_artist', { artist });
}

export async function removeArtist(artistId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_artist', { artistId });
}