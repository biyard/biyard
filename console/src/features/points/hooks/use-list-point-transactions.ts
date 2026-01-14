import { useQuery } from "@tanstack/react-query";
import { apiClient } from "../../../lib/api-client";
import { PointTransaction } from "../models/point-transaction";
import type { ListPointTransactionsResponse } from "../dto/point-transaction-response";

export function useListPointTransactions(projectId: string) {
  return useQuery({
    queryKey: ["point-transactions", projectId],
    queryFn: async () => {
      const response = await apiClient.get<ListPointTransactionsResponse>(
        `/v1/projects/${projectId}/points/transactions`
      );
      return {
        transactions: PointTransaction.fromResponses(response.items),
        bookmark: response.bookmark,
      };
    },
    enabled: !!projectId,
  });
}
