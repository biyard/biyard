import type { PointAggregationResponse } from "../dto/point-aggregation-response";

export class PointAggregation {
  public readonly date: string;
  public readonly suppliedPoints: number;
  public readonly tradedPoints: number;
  public readonly awardedPoints: number;
  public readonly deductedPoints: number;
  public readonly exchangedPoints: number;

  constructor(data: PointAggregationResponse) {
    this.date = data.date;
    this.suppliedPoints = data.supplied_points;
    this.tradedPoints = data.traded_points;
    this.awardedPoints = data.awarded_points;
    this.deductedPoints = data.deducted_points;
    this.exchangedPoints = data.exchanged_points;
  }

  getFormattedAwarded(): string {
    return `+${this.awardedPoints.toLocaleString()}`;
  }

  getFormattedDeducted(): string {
    return `-${Math.abs(this.deductedPoints).toLocaleString()}`;
  }

  static fromResponse(response: PointAggregationResponse): PointAggregation {
    return new PointAggregation(response);
  }

  static fromResponses(responses: PointAggregationResponse[]): PointAggregation[] {
    return responses.map((response) => new PointAggregation(response));
  }
}
