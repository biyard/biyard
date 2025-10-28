import { useMutation } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { AccountResponse } from '../dto/account-response';

export function useWithdrawal() {
  return useMutation({
    mutationFn: async () => {
      return apiClient.post<AccountResponse>('/v1/accounts/withdrawal');
    },
  });
}
