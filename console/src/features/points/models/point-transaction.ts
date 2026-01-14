import type {
  PointTransactionResponse,
  TransactionType,
} from "../dto/point-transaction-response";
import { fromMillis, formatDateTime } from "@/lib/date";

export class PointTransaction {
  public readonly projectId: string;
  public readonly metaUserId: string;
  public readonly month: string;
  public readonly transactionType: TransactionType;
  public readonly amount: number;
  public readonly targetUserId?: string;
  public readonly description?: string;
  public readonly createdAt: Date;

  constructor(data: PointTransactionResponse) {
    this.projectId = data.project_id;
    this.metaUserId = data.meta_user_id;
    this.month = data.month;
    this.transactionType = data.transaction_type;
    this.amount = data.amount;
    this.targetUserId = data.target_user_id;
    this.description = data.description;
    this.createdAt = fromMillis(data.created_at);
  }

  isPositive(): boolean {
    return this.amount > 0;
  }

  isNegative(): boolean {
    return this.amount < 0;
  }

  getFormattedAmount(): string {
    const prefix = this.amount > 0 ? "+" : "";
    return `${prefix}${this.amount.toLocaleString()}`;
  }

  getFormattedCreatedAt(): string {
    return formatDateTime(this.createdAt);
  }

  getTransactionTypeLabel(): string {
    switch (this.transactionType) {
      case "Award":
        return "Award";
      case "Deduct":
        return "Deduct";
      case "Transfer":
        return "Transfer";
      case "Exchange":
        return "Exchange";
      default:
        return this.transactionType;
    }
  }

  getTransactionTypeColor(): string {
    switch (this.transactionType) {
      case "Award":
        return "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200";
      case "Deduct":
        return "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200";
      case "Transfer":
        return "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200";
      case "Exchange":
        return "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200";
      default:
        return "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200";
    }
  }

  static fromResponse(response: PointTransactionResponse): PointTransaction {
    return new PointTransaction(response);
  }

  static fromResponses(responses: PointTransactionResponse[]): PointTransaction[] {
    return responses.map((response) => new PointTransaction(response));
  }
}
