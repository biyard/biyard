import { useMutation } from "@tanstack/react-query";
import { apiClient } from "@/lib/api-client";

export interface ActivityRequest {
  meta_user_id: string;
  activity_type: string;
  value: number;
  description: string;
}

export interface ActivityResponse {
  points_earned: number;
  total_points: number;
}

export function useCreateActivity(projectId: string) {
  return useMutation({
    mutationFn: (data: ActivityRequest) =>
      apiClient.post<ActivityResponse>(
        `/v1/projects/${projectId}/activities`,
        data,
      ),
  });
}
