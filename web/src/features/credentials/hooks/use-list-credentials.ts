import { useQuery } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { CredentialResponse } from './use-create-credential';

export function useListCredentials() {
  return useQuery({
    queryKey: ['credentials'],
    queryFn: async () => {
      return apiClient.get<CredentialResponse[]>('/v1/credentials');
    },
  });
}
