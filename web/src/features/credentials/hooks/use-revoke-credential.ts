import { useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { CredentialResponse } from './use-create-credential';

export function useRevokeCredential() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (credentialId: string) => {
      return apiClient.delete<CredentialResponse>(`/v1/credentials/${credentialId}`);
    },
    onSuccess: () => {
      // Invalidate credentials list to refetch
      queryClient.invalidateQueries({ queryKey: ['credentials'] });
    },
  });
}
