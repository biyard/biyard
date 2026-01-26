import { config } from "@/config";
import { logger } from "@/lib/logger";

export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";

export async function call<T, R>(
  method: HttpMethod,
  path: string,
  body?: T,
): Promise<R | undefined> {
  const apiBaseUrl: string = config.apiBaseUrl;
  let headers = undefined;

  let response;
  if (body) {
    if (!headers) {
      headers = new Headers();
    }
    headers.set("Content-Type", "application/json");
    response = await fetch(`${apiBaseUrl}${path}`, {
      method,
      headers,
      credentials: "include",
      body: body ? JSON.stringify(body) : undefined,
    });
  } else {
    response = await fetch(`${apiBaseUrl}${path}`, {
      method,
      headers,
      credentials: "include",
    });
  }

  if (!response.ok) {
    const errorData = await response
      .json()
      .catch((e) => `Failed to fetch and parse error ${e}`);
    logger.error("request error on call", errorData);

    throw new ServiceError(errorData);
  }

  try {
    const json_body: R = await response.json();
    logger.debug(
      `API Response Body(${method} ${apiBaseUrl}${path}): `,
      json_body,
    );

    return json_body;
  } catch (e) {
    logger.debug("failed to parse response body as json", e);

    return undefined;
  }
}

export class ServiceError extends Error {
  readonly name: string;
  readonly message: string;

  constructor(message: string) {
    super(message);
    this.name = "RatelSdkError";
    this.message = message;
  }
}
