export type TransactionType = "Award" | "Deduct" | "Transfer" | "Exchange";

export interface PointTransactionResponse {
  project_id: string;
  meta_user_id: string;
  month: string;
  transaction_type: TransactionType;
  amount: number;
  target_user_id?: string;
  description?: string;
  created_at: number;
}

export interface ListPointTransactionsResponse {
  items: PointTransactionResponse[];
  bookmark?: string;
}
