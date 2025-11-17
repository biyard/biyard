import { useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import { useProjectToasts } from './use-project-toasts';

export function useDeleteProject() {
  const queryClient = useQueryClient();
  const toasts = useProjectToasts();

  return useMutation({
    mutationFn: async (projectId: string) => {
      return apiClient.delete(`/v1/projects/${projectId}`);
    },
    onSuccess: () => {
      // Show success toast
      toasts.showDeleteSuccess();

      // Invalidate and refetch projects list
      // Using refetchType: 'active' ensures the active query is refetched immediately
      queryClient.invalidateQueries({
        queryKey: ['projects'],
        refetchType: 'active'
      });
    },
    onError: () => {
      // Show error toast
      toasts.showDeleteError();
    },
  });
}
