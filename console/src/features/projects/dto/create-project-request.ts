export interface CreateProjectRequest {
  name: string;
  description?: string;
  monthly_token_supply: number;

  // These fields are sent to backend but will be managed automatically
  // Points are dynamically supplied via transaction APIs
  // Exchange ratio is automatically calculated
  monthly_points_supply?: number;
  exchange_ratio?: number;
}
