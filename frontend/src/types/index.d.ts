import {tauri} from "@tauri-apps/api";

export {};

declare global {
    interface Window {
        __TAURI__: typeof tauri | undefined // 👈️ extend window by this property
    }

    interface Array<T> {
        groupBy<T, K extends keyof any>(key: (i: T) => K): Record<K, T[]>
    }
}
