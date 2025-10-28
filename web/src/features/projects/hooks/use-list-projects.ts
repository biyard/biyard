import { useQuery } from '@tanstack/react-query';
import { useEffect } from 'react';
import { apiClient } from '../../../lib/api-client';
import { useProjectToasts } from './use-project-toasts';
import type { ProjectResponse } from '../dto/project-response';

interface ListProjectsResponse {
  items: ProjectResponse[];
  bookmark?: string;
}

export function useListProjects() {
  const toasts = useProjectToasts();

  const query = useQuery({
    queryKey: ['projects'],
    queryFn: async () => {
      const response = await apiClient.get<ListProjectsResponse>('/v1/projects');
      return response.items;
    },
  });

  useEffect(() => {
    if (query.isError) {
      toasts.showLoadError();
    }
  }, [query.isError, toasts]);

  return query;
}
