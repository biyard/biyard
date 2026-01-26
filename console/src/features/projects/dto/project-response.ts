export interface ProjectResponse {
  id: string;
  account_id: string;
  name: string;
  description?: string;
  monthly_points_supply?: number;
  monthly_token_supply?: number;
  exchange_ratio?: number;
  token_value?: number;
  status: string;
  created_at: number;
  updated_at: number;
}
