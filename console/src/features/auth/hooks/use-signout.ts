import { useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';

export function useSignout() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async () => {
      return apiClient.post<void>('/v1/accounts/signout');
    },
    onSuccess: () => {
      // Invalidate session query to clear cached session data
      queryClient.invalidateQueries({ queryKey: ['session'] });
      // Clear all queries to ensure fresh state after signout
      queryClient.clear();
    },
  });
}
