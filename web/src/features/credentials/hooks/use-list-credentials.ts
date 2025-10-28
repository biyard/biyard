import { useQuery } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import { Credential } from '../models/credential';
import type { CredentialResponse } from '../dto/credential-response';

export function useListCredentials() {
  return useQuery({
    queryKey: ['credentials'],
    queryFn: async () => {
      const responses = await apiClient.get<CredentialResponse[]>('/v1/credentials');
      return Credential.fromResponses(responses);
    },
  });
}
