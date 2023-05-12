import { createEmpty, type User } from "$lib/model/user/User";
import { writable } from "svelte/store";

export const ACCOUNT_STORAGE_KEY = 'account';
const defaultUser = createEmpty();

export const accountStore = writable(getCurrentAccount());

export function getCurrentAccount() {
    return <User>JSON.parse(localStorage.getItem(ACCOUNT_STORAGE_KEY) || JSON.stringify(defaultUser));
}

export function setCurrentAccount(user: User) {
    localStorage.setItem(ACCOUNT_STORAGE_KEY, JSON.stringify(user));
    accountStore.set(user);
}

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