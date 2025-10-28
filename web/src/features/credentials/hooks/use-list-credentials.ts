import { useQuery } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import type { CredentialResponse } from '../dto/credential-response';

export function useListCredentials() {
  return useQuery({
    queryKey: ['credentials'],
    queryFn: async () => {
      return apiClient.get<CredentialResponse[]>('/v1/credentials');
    },
  });
}
