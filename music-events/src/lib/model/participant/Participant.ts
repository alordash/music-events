import { sleepMaxOneSec } from "$lib/Timer";
import { FieldInfo, FieldInfoUnknown, exploreComposer } from "$lib/generic_object_form/FieldInfo";
import { FieldTypes } from "$lib/generic_object_form/FieldTypes";
import { invoke } from "@tauri-apps/api/tauri";
import { getConcertById, getConcertsCount, getConcertsPaginated } from "../concert/Concert";
import { getGroupById, getGroupsCount, getGroupsPaginated } from "../group/Group";

export const PARTICIPANT_ID_LITERAL = 'participant_id';

export type Participant = {
    id: number,
    concertId: number,
    groupId: number
}

export function fieldComposer(fieldName: string): FieldInfo {
    switch (fieldName) {
        case 'id':
            return FieldInfo('Id', FieldTypes.Id);
        case 'concertId':
            return FieldInfo(
                'Concert',
                FieldTypes.ObjectReference,
                10000,
                getConcertById,
                exploreComposer(getConcertsPaginated),
                getConcertsCount,
                fieldComposer,
                "concert"
            );
        case 'groupId':
            return FieldInfo(
                'Group',
                FieldTypes.ObjectReference,
                20000,
                getGroupById,
                exploreComposer(getGroupsPaginated),
                getGroupsCount,
                fieldComposer,
                "group"
            );

        default:
            return FieldInfoUnknown();
    }
}

export function createEmpty(): Participant {
    return {
        id: 0,
        concertId: 0,
        groupId: 0
    }
}

export async function createParticipant({ concertId, groupId }: Participant): Promise<object> {
    await sleepMaxOneSec();
    return invoke('create_participant', { concertId, groupId })
}

export async function getParticipantsPaginated(count: number, offset: number): Promise<Array<Participant>> {
    await sleepMaxOneSec();
    return await invoke('get_participants_paginated', { count, offset });
}

export async function getAllParticipants(): Promise<Array<Participant>> {
    await sleepMaxOneSec();
    return invoke('get_all_participants');
}

export async function getParticipantsCount(): Promise<number> {
    await sleepMaxOneSec();
    return invoke('get_participants_count');
}

export async function getAllParticipantIds(): Promise<Array<number>> {
    await sleepMaxOneSec();
    return invoke('get_all_participant_ids');
}

export async function getParticipantById(participantId: number): Promise<Participant | null> {
    await sleepMaxOneSec();
    return invoke('get_participant_by_id', { participantId });
}

export async function addParticipant(participant: object): Promise<number> {
    await sleepMaxOneSec();
    return invoke('add_participant', { participant });
}

export async function updateParticipant(participant: Participant): Promise<void> {
    await sleepMaxOneSec();
    return invoke('update_participant', { participant });
}

export async function removeParticipant(participantId: number): Promise<void> {
    await sleepMaxOneSec();
    return invoke('remove_participant', { participantId });
}