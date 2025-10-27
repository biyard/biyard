import { useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';

export interface CreateCredentialRequest {
  name: string;
}

export interface CredentialResponse {
  pk: string;
  name: string;
  api_key_prefix: string;
  status: 'Active' | 'Revoked';
  created_at: number;
  last_used_at?: number;
  api_key?: string; // Only present on creation
}

export function useCreateCredential() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: CreateCredentialRequest) => {
      return apiClient.post<CredentialResponse>('/v1/credentials', data);
    },
    onSuccess: () => {
      // Invalidate credentials list to refetch
      queryClient.invalidateQueries({ queryKey: ['credentials'] });
    },
  });
}
