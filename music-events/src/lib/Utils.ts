export function GenNumRange(start: number, end: number) {
    return [...Array(end - start).keys()].map(v => v + start);
}