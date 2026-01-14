import type { TokenResponse } from "../dto/token-response";
import { fromMillis, formatDateTime } from "@/lib/date";

export class Token {
  public readonly pk: string;
  public readonly name: string;
  public readonly symbol: string;
  public readonly decimals: number;
  public readonly totalSupply: number;
  public readonly circulatingSupply: number;
  public readonly description?: string;
  public readonly createdAt: Date;
  public readonly updatedAt: Date;

  constructor(data: TokenResponse) {
    this.pk = data.pk;
    this.name = data.name;
    this.symbol = data.symbol;
    this.decimals = data.decimals;
    this.totalSupply = data.total_supply;
    this.circulatingSupply = data.circulating_supply;
    this.description = data.description;
    this.createdAt = fromMillis(data.created_at);
    this.updatedAt = fromMillis(data.updated_at);
  }

  getFormattedTotalSupply(): string {
    return this.totalSupply.toLocaleString();
  }

  getFormattedCirculatingSupply(): string {
    return this.circulatingSupply.toLocaleString();
  }

  getFormattedCreatedAt(): string {
    return formatDateTime(this.createdAt);
  }

  static fromResponse(response: TokenResponse): Token {
    return new Token(response);
  }

  static fromResponses(responses: TokenResponse[]): Token[] {
    return responses.map((response) => new Token(response));
  }
}
