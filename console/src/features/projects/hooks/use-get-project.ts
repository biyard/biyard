import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { apiClient } from "../../../lib/api-client";
import { useProjectToasts } from "./use-project-toasts";
import { Project } from "../models/project";
import type { ProjectResponse } from "../dto/project-response";

export function useGetProject(projectId: string) {
  const toasts = useProjectToasts();

  const query = useQuery({
    queryKey: ["projects", projectId],
    queryFn: async () => {
      const response = await apiClient.get<ProjectResponse>(
        `/v1/projects/${projectId}`
      );
      return Project.fromResponse(response);
    },
    enabled: !!projectId,
  });

  useEffect(() => {
    if (query.isError) {
      toasts.showLoadError();
    }
  }, [query.isError, toasts]);

  return query;
}
