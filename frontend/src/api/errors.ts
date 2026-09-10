import type { StructuredError } from "./types";

export class ApiError extends Error {
  readonly structured?: StructuredError;

  constructor(message: string, structured?: StructuredError) {
    super(message);
    this.name = "ApiError";
    this.structured = structured;
  }

  static fromUnknown(error: unknown): ApiError {
    if (error instanceof ApiError) {
      return error;
    }

    if (typeof error === "object" && error !== null && "message" in error) {
      const structured = error as StructuredError;
      return new ApiError(String(structured.message), structured);
    }

    return new ApiError(String(error));
  }
}
