import { useQuery } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { AccountResponse } from '../dto/account-response';

export function useValidateSession() {
  return useQuery({
    queryKey: ['session'],
    queryFn: async () => {
      try {
        return await apiClient.get<AccountResponse>('/v1/accounts/me');
      } catch (error) {
        // Session is invalid or expired
        return null;
      }
    },
    retry: false,
    staleTime: Infinity, // Don't refetch automatically
  });
}
