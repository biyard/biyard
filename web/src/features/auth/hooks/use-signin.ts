import { useMutation } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { SigninRequest } from '../dto/signin-request';
import type { AccountResponse } from '../dto/account-response';

export function useSignin() {
  return useMutation({
    mutationFn: async (data: SigninRequest) => {
      return apiClient.post<AccountResponse>('/v1/accounts/signin', data);
    },
  });
}
