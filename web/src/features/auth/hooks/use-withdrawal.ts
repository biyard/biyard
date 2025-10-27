import { useMutation } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { AccountResponse } from '../../../types/account';

export function useWithdrawal() {
  return useMutation({
    mutationFn: async () => {
      return apiClient.post<AccountResponse>('/v1/accounts/withdrawal');
    },
  });
}
