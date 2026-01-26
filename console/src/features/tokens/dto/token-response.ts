export interface TokenResponse {
  pk: string;
  name: string;
  symbol: string;
  decimals: number;
  total_supply: number;
  circulating_supply: number;
  description?: string;
  created_at: number;
  updated_at: number;
}

export interface ListTokensResponse {
  tokens: TokenResponse[];
}
