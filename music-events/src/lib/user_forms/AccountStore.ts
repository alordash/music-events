import { writable } from "svelte/store";

export const ACCOUNT_DEFAULT_ROLE = 'guest';
export const accountRoleStore = writable(ACCOUNT_DEFAULT_ROLE);

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