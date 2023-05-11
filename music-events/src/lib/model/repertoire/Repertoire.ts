import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown, exploreComposer } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { invoke } from "@tauri-apps/api/tauri";
import { getParticipantById, getParticipantsCount, getParticipantsPaginated, nameComposer as participantNameComposer } from "../participant/Participant";
export const REPERTOIRE_ID_LITERAL = 'repertoire_id';

export type Repertoire = {
    id: number,
    name: string,
    orderNumber: number,
    participantId: number
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'name':
            return FieldInfo('Name', FieldTypes.Name);
        case 'orderNumber':
            return FieldInfo('Order number', FieldTypes.Number);
        case 'participantId':
            return FieldInfo(
                'Participant',
                FieldTypes.ObjectReference,
                20000,
                getParticipantById,
                exploreComposer(getParticipantsPaginated),
                getParticipantsCount,
                fieldComposer,
                "participant",
                participantNameComposer
            );

        default:
            return FieldInfoUnknown();
    }
}

export function createEmpty(): Repertoire {
    return {
        id: 0,
        name: '',
        orderNumber: 0,
        participantId: 0
    }
}

export async function createRepertoire({ name, orderNumber, participantId }: Repertoire): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_repertoire', { name, orderNumber, participantId })
}

export async function getRepertoiresPaginated(count: number, offset: number): Promise<Array<Repertoire>> {
    await sleepMaxOneSec();
    return await invoke('get_repertoires_paginated', { count, offset });
}

export async function getAllRepertoires(): Promise<Array<Repertoire>> {
    await sleepMaxOneSec();
    return invoke('get_all_repertoires');
}

export async function getRepertoiresCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_repertoires_count');
}

export async function getAllRepertoireIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_repertoire_ids');
}

export async function getRepertoireById(repertoireId: number): Promise<Repertoire | null> {
    await sleepMaxOneSec();
    return invoke('get_repertoire_by_id', { repertoireId });
}

export async function addRepertoire(repertoire: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_repertoire', { repertoire });
}

export async function updateRepertoire(repertoire: Repertoire): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_repertoire', { repertoire });
}

export async function removeRepertoire(repertoireId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_repertoire', { repertoireId });
}