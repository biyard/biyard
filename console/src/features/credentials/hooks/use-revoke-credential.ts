import { useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { CredentialResponse } from '../dto/credential-response';

export function useRevokeCredential() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (credentialId: string) => {
      return apiClient.delete<CredentialResponse>(`/v1/credentials/${credentialId}`);
    },
    onSuccess: async () => {
      // Invalidate and refetch credentials list
      await queryClient.invalidateQueries({ queryKey: ['credentials'] });
    },
  });
}
