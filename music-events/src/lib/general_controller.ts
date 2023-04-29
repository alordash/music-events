import { invoke } from "@tauri-apps/api/tauri";
import { sleepMaxOneSec } from "./timer";

export async function transactionCommit(transactionId: object): Promise<void> {
    await sleepMaxOneSec();
    return invoke('transaction_commit', { transactionId });
}

export async function transactionRollback(transactionId: object): Promise<void> {
    await sleepMaxOneSec();
    return invoke('transaction_rollback', { transactionId });
}