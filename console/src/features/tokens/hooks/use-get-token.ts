import { useQuery } from "@tanstack/react-query";
import { apiClient } from "../../../lib/api-client";
import { Token } from "../models/token";
import type { TokenResponse } from "../dto/token-response";

export function useGetToken(projectId: string) {
  return useQuery({
    queryKey: ["token", projectId],
    queryFn: async () => {
      const response = await apiClient.get<TokenResponse>(
        `/v1/projects/${projectId}/tokens`
      );
      return Token.fromResponse(response);
    },
    enabled: !!projectId,
  });
}
