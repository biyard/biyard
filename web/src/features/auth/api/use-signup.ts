import { useMutation } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { SignupRequest, AccountResponse } from '../../../types/account';

export function useSignup() {
  return useMutation({
    mutationFn: async (data: SignupRequest) => {
      return apiClient.post<AccountResponse>('/v1/accounts/signup', data);
    },
  });
}
