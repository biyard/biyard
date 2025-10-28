import { useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '../../../lib/api-client';
import { useProjectToasts } from './use-project-toasts';
import type { CreateProjectRequest } from '../dto/create-project-request';
import type { ProjectResponse } from '../dto/project-response';

export function useCreateProject() {
  const queryClient = useQueryClient();
  const toasts = useProjectToasts();

  return useMutation({
    mutationFn: async (data: CreateProjectRequest) => {
      return apiClient.post<ProjectResponse>('/v1/projects', data);
    },
    onSuccess: () => {
      // Show success toast
      toasts.showCreateSuccess();

      // Invalidate and refetch projects list
      // Using refetchType: 'active' ensures the active query is refetched immediately
      queryClient.invalidateQueries({
        queryKey: ['projects'],
        refetchType: 'active'
      });
    },
    onError: () => {
      // Show error toast
      toasts.showCreateError();
    },
  });
}
