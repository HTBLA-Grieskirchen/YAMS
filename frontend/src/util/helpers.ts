import humanizeDuration from "humanize-duration";

export function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export function groupBy<T, K extends keyof any>(array: Array<T>, key: (i: T) => K) {
    return array.reduce((groups, item) => {
        (groups[key(item)] ||= []).push(item);
        return groups;
    }, {} as Record<K, T[]>)
}

export function formatDuration(ms: number, language: string): string {
    return humanizeDuration(ms, {
        language: language,
        fallbacks: ["en"],
        largest: 2,
        round: true,
    })
}

export function capitalize(word: string) {
    if (!word) return word;
    return word[0].toUpperCase() + word.slice(1);
}

export function titleCase(word: string) {
    return capitalize(word.toLowerCase())
}
