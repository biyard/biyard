import { CredentialStatus } from "../types/credential-status";
import type { CredentialResponse } from "../dto/credential-response";

/**
 * Credential domain model
 * Encapsulates credential business logic and state management
 */
export class Credential {
  public readonly id: string;
  public readonly name: string;
  public readonly apiKeyPrefix: string;
  public readonly status: CredentialStatus;
  public readonly createdAt: Date;
  public readonly lastUsedAt?: Date;
  public readonly apiKey?: string; // Only present on creation

  constructor(data: CredentialResponse) {
    this.id = data.id;
    this.name = data.name;
    this.apiKeyPrefix = data.api_key_prefix;
    this.status = data.status;
    this.createdAt = new Date(data.created_at * 1000);
    this.lastUsedAt = data.last_used_at ? new Date(data.last_used_at * 1000) : undefined;
    this.apiKey = data.api_key;
  }

  /**
   * Check if credential is active
   */
  isActive(): boolean {
    return this.status === CredentialStatus.Active;
  }

  /**
   * Check if credential is revoked
   */
  isRevoked(): boolean {
    return this.status === CredentialStatus.Revoked;
  }

  /**
   * Get status display class for UI styling
   */
  getStatusColorClass(): string {
    return this.isActive()
      ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
      : "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200";
  }

  /**
   * Format created date for display
   */
  getFormattedCreatedAt(): string {
    return this.createdAt.toLocaleString();
  }

  /**
   * Format last used date for display
   */
  getFormattedLastUsedAt(): string {
    return this.lastUsedAt ? this.lastUsedAt.toLocaleString() : "Never";
  }

  /**
   * Create Credential instance from API response
   */
  static fromResponse(response: CredentialResponse): Credential {
    return new Credential(response);
  }

  /**
   * Create multiple Credential instances from API responses
   */
  static fromResponses(responses: CredentialResponse[]): Credential[] {
    return responses.map((response) => new Credential(response));
  }
}
