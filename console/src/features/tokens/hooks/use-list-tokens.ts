import { useQuery } from "@tanstack/react-query";
import { apiClient } from "../../../lib/api-client";
import { Token } from "../models/token";
import type { ListTokensResponse } from "../dto/token-response";

export function useListTokens(projectId: string) {
  return useQuery({
    queryKey: ["tokens", projectId],
    queryFn: async () => {
      const response = await apiClient.get<ListTokensResponse>(
        `/v1/projects/${projectId}/tokens`
      );
      return Token.fromResponses(response.tokens);
    },
    enabled: !!projectId,
  });
}
