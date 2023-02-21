export function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export function groupBy<T, K extends keyof any>(array: Array<T>, key: (i: T) => K) {
    return array.reduce((groups, item) => {
        (groups[key(item)] ||= []).push(item);
        return groups;
    }, {} as Record<K, T[]>)
}

