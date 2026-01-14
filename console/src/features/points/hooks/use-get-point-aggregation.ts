import { useQuery } from "@tanstack/react-query";
import { apiClient } from "@/lib/api-client";
import type { PointAggregationResponse } from "../dto/point-aggregation-response";
import { PointAggregation } from "../models/point-aggregation";

export function useGetPointAggregation(projectId: string, date: string) {
  return useQuery({
    queryKey: ["pointAggregation", projectId, date],
    queryFn: async () => {
      const response = await apiClient.get<PointAggregationResponse>(
        `/v1/projects/${projectId}/points?date=${encodeURIComponent(date)}`
      );
      return PointAggregation.fromResponse(response);
    },
    enabled: !!projectId && !!date,
  });
}
