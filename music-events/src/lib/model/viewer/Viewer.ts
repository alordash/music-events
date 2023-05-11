import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown, exploreComposer } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { invoke } from "@tauri-apps/api/tauri";
import { getPersonById, getPersonsCount, getPersonsPaginated } from "../person/Person";
import { getViewerSeatById, getViewerSeatsCount, getViewerSeatsPaginated, nameComposer } from "../viewer_seat/ViewerSeat";

export const VIEWER_ID_LITERAL = 'viewer_id';

export type Viewer = {
    id: number,
    personId: number,
    viewerSeatId: number
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
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
        case 'viewerSeatId':
            return FieldInfo(
                'Viewer\'s seat',
                FieldTypes.ObjectReference,
                20000,
                getViewerSeatById,
                exploreComposer(getViewerSeatsPaginated),
                getViewerSeatsCount,
                fieldComposer,
                "viewer seats",
                nameComposer
            );

        default:
            return FieldInfoUnknown();
    }
}

export function createEmpty(): Viewer {
    return {
        id: 0,
        personId: 0,
        viewerSeatId: 0
    }
}

export async function createViewer({ personId, viewerSeatId }: Viewer): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_viewer', { personId, viewerSeatId })
}

export async function getViewersPaginated(count: number, offset: number): Promise<Array<Viewer>> {
    await sleepMaxOneSec();
    return await invoke('get_viewers_paginated', { count, offset });
}

export async function getAllViewers(): Promise<Array<Viewer>> {
    await sleepMaxOneSec();
    return invoke('get_all_viewers');
}

export async function getViewersCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_viewers_count');
}

export async function getAllViewerIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_viewer_ids');
}

export async function getViewerById(viewerId: number): Promise<Viewer | null> {
    await sleepMaxOneSec();
    return invoke('get_viewer_by_id', { viewerId });
}

export async function addViewer(viewer: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_viewer', { viewer });
}

export async function updateViewer(viewer: Viewer): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_viewer', { viewer });
}

export async function removeViewer(viewerId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_viewer', { viewerId });
}