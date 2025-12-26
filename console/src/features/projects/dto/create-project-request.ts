export interface CreateProjectRequest {
  name: string;
  description?: string;
  monthly_token_supply: number;
  symbol: string;
  decimals?: number;
}
