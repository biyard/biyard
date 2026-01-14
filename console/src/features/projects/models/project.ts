import type { ProjectResponse } from "../dto/project-response";
import { fromMillis, formatDateTime } from "@/lib/date";

/**
 * Project domain model
 * Encapsulates project business logic and state management
 */
export class Project {
  public readonly id: string;
  public readonly accountId: string;
  public readonly name: string;
  public readonly description?: string;
  public readonly monthlyTokenSupply: number;
  public readonly exchangeRatio: number;
  public readonly tokenValue: number;
  public readonly status: string;
  public readonly createdAt: Date;
  public readonly updatedAt: Date;

  constructor(data: ProjectResponse) {
    this.id = data.id;
    this.accountId = data.account_id;
    this.name = data.name;
    this.description = data.description;
    this.monthlyTokenSupply = data.monthly_token_supply ?? 0;
    this.exchangeRatio = data.exchange_ratio ?? 1;
    this.tokenValue = data.token_value ?? 0;
    this.status = data.status;
    this.createdAt = fromMillis(data.created_at);
    this.updatedAt = fromMillis(data.updated_at);
  }

  isActive(): boolean {
    return this.status === "active";
  }

  hasAutomaticTokenSupply(): boolean {
    return this.monthlyTokenSupply > 0;
  }

  hasManualTokenSupply(): boolean {
    return this.monthlyTokenSupply === 0;
  }

  getStatusColorClass(): string {
    return this.isActive()
      ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
      : "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200";
  }

  getFormattedTokenSupply(): string {
    if (this.hasManualTokenSupply()) {
      return "Manual";
    }
    return this.monthlyTokenSupply.toLocaleString();
  }


  getFormattedTokenValue(): string {
    return this.tokenValue.toFixed(4);
  }

  getFormattedCreatedAt(): string {
    return formatDateTime(this.createdAt);
  }

  getFormattedUpdatedAt(): string {
    return formatDateTime(this.updatedAt);
  }

  static fromResponse(response: ProjectResponse): Project {
    return new Project(response);
  }

  static fromResponses(responses: ProjectResponse[]): Project[] {
    return responses.map((response) => new Project(response));
  }
}
