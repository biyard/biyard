import { useMutation } from "@tanstack/react-query";
import { apiClient } from "@/lib/api-client";

export interface PurchaseRequest {
  meta_user_id: string;
  amount: number;
  item_name: string;
  reward_rate: number;
}

export interface PurchaseResponse {
  purchase_amount: number;
  reward_points: number;
  treasury_contribution: number;
}

export function useCreatePurchase(projectId: string) {
  return useMutation({
    mutationFn: (data: PurchaseRequest) =>
      apiClient.post<PurchaseResponse>(
        `/v1/projects/${projectId}/purchases`,
        data,
      ),
  });
}
