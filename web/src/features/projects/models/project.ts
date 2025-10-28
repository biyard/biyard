import type { ProjectResponse } from "../dto/project-response";

/**
 * Project domain model
 * Encapsulates project business logic and state management
 */
export class Project {
  public readonly id: string;
  public readonly accountId: string;
  public readonly name: string;
  public readonly description?: string;
  public readonly monthlyPointsSupply: number;
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
    this.monthlyPointsSupply = data.monthly_points_supply;
    this.monthlyTokenSupply = data.monthly_token_supply;
    this.exchangeRatio = data.exchange_ratio;
    this.tokenValue = data.token_value;
    this.status = data.status;
    this.createdAt = new Date(data.created_at * 1000);
    this.updatedAt = new Date(data.updated_at * 1000);
  }

  /**
   * Check if project is active
   */
  isActive(): boolean {
    return this.status === "active";
  }

  /**
   * Check if monthly token supply is enabled
   */
  hasAutomaticTokenSupply(): boolean {
    return this.monthlyTokenSupply > 0;
  }

  /**
   * Check if monthly token supply is manual (zero)
   */
  hasManualTokenSupply(): boolean {
    return this.monthlyTokenSupply === 0;
  }

  /**
   * Get status display class for UI styling
   */
  getStatusColorClass(): string {
    return this.isActive()
      ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
      : "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200";
  }

  /**
   * Format monthly token supply for display
   */
  getFormattedTokenSupply(): string {
    if (this.hasManualTokenSupply()) {
      return "Manual";
    }
    return this.monthlyTokenSupply.toLocaleString();
  }

  /**
   * Format monthly points supply for display
   */
  getFormattedPointsSupply(): string {
    return this.monthlyPointsSupply.toLocaleString();
  }

  /**
   * Format token value for display
   */
  getFormattedTokenValue(): string {
    return this.tokenValue.toFixed(4);
  }

  /**
   * Format created date for display
   */
  getFormattedCreatedAt(): string {
    return this.createdAt.toLocaleString();
  }

  /**
   * Format updated date for display
   */
  getFormattedUpdatedAt(): string {
    return this.updatedAt.toLocaleString();
  }

  /**
   * Create Project instance from API response
   */
  static fromResponse(response: ProjectResponse): Project {
    return new Project(response);
  }

  /**
   * Create multiple Project instances from API responses
   */
  static fromResponses(responses: ProjectResponse[]): Project[] {
    return responses.map((response) => new Project(response));
  }
}
