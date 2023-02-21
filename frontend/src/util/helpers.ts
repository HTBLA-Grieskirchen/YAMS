export function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

Array.prototype.groupBy = function <T, K extends keyof any>(key: (i: T) => K) {
    return this.reduce((groups, item) => {
        (groups[key(item)] ||= []).push(item);
        return groups;
    }, {} as Record<K, T[]>)
}

