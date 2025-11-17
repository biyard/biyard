import { useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { CreateCredentialRequest } from '../dto/create-credential-request';
import type { CredentialResponse } from '../dto/credential-response';

export function useCreateCredential() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: CreateCredentialRequest) => {
      return apiClient.post<CredentialResponse>('/v1/credentials', data);
    },
    onSuccess: async () => {
      // Invalidate and refetch credentials list
      await queryClient.invalidateQueries({ queryKey: ['credentials'] });
    },
  });
}
