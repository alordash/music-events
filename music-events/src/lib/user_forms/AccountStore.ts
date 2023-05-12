import { createEmpty } from "$lib/model/user/User";
import { writable } from "svelte/store";

const defaultUser = createEmpty();
export const accountStore = writable(defaultUser);

export function roleMapper(role: string) {
    switch (role) {
        case 'admin':
            return '/admin/';
        case 'stuff':
            return '/stuff/';
        case 'client':
            return '/client/';
        default:
            return '/';
    }
}