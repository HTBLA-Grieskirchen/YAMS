import createClient from "openapi-fetch";
import type { paths, components } from "./schema";
import { invoke } from "@tauri-apps/api/core";

export type Address = components["schemas"]["AddressDTO"];

export interface BackendClient {
  getAddresses(): Promise<Address[]>;
  createAddress(address: Address): Promise<Address>;
}

export class HttpBackendClient implements BackendClient {
  private client: ReturnType<typeof createClient<paths>>;

  constructor(baseUrl: string) {
    this.client = createClient<paths>({ baseUrl });
  }

  async getAddresses(): Promise<Address[]> {
    const { data, error } = await this.client.GET("/addresses");
    if (error) throw error;
    // openapi-fetch might wrap the data in a content-type key if not careful
    // but usually it handles JSON automatically if specified in spec.
    // Based on schema.d.ts, it's under "application/json; charset=utf-8"
    return data as Address[];
  }

  async createAddress(address: Address): Promise<Address> {
    const { data, error } = await this.client.POST("/addresses", {
      body: address as any, // Cast to any to avoid strict body matching issues if schema differs slightly
    });
    if (error) throw error;
    return data as Address;
  }
}

export class TauriBackendClient implements BackendClient {
  async getAddresses(): Promise<Address[]> {
    return await invoke<Address[]>("get_addresses");
  }

  async createAddress(address: Address): Promise<Address> {
    return await invoke<Address>("create_address", { address });
  }
}

export const getBackendClient = (config: { mode: 'standalone' | 'embedded', serverUrl?: string }): BackendClient => {
  if (config.mode === 'standalone' && config.serverUrl) {
    return new HttpBackendClient(config.serverUrl);
  }
  return new TauriBackendClient();
};
