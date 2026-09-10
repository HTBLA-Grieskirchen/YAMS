import { tauri } from "@tauri-apps/api";

export {};

declare global {
  interface Window {
    __TAURI__: typeof tauri | undefined; // 👈️ extend window by this property
  }

  interface Navigator {
    userLanguage: string | undefined;
  }
}
