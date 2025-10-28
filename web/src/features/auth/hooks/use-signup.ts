import { useMutation } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { SignupRequest } from '../dto/signup-request';
import type { AccountResponse } from '../dto/account-response';

export function useSignup() {
  return useMutation({
    mutationFn: async (data: SignupRequest) => {
      return apiClient.post<AccountResponse>('/v1/accounts/signup', data);
    },
  });
}
