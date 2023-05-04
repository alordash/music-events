export async function sleep(ms: number) {
    await new Promise(resolve => setTimeout(resolve, ms));
}

export async function sleepMaxOneSec() {
    await sleep(Math.random() * 2000 + 1000);
}