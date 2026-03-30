import { useQuery } from "@tanstack/react-query";
import { apiClient } from "@/lib/api-client";

export interface TreasuryResponse {
  total_treasury: number;
  floor_price: number;
  total_supply: number;
  circulating_supply: number;
}

export function useGetTreasury(projectId: string | undefined) {
  return useQuery({
    queryKey: ["treasury", projectId],
    queryFn: () =>
      apiClient.get<TreasuryResponse>(`/v1/projects/${projectId}/treasury`),
    enabled: !!projectId,
  });
}
