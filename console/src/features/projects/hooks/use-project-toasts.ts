import { toast } from "sonner";
import { useProjectsPageI18n } from "../components/ProjectsPage/i18n";

/**
 * Custom hook for showing toast notifications for project operations
 */
export function useProjectToasts() {
  const t = useProjectsPageI18n();

  return {
    showLoadError: () => {
      toast.error(t.errorLoadingProjects, {
        description: t.errorLoadingProjectsDescription,
      });
    },
    showCreateError: () => {
      toast.error(t.errorCreatingProject, {
        description: t.errorCreatingProjectDescription,
      });
    },
    showCreateSuccess: () => {
      toast.success(t.projectCreatedSuccess);
    },
    showDeleteError: () => {
      toast.error(t.errorDeletingProject, {
        description: t.errorDeletingProjectDescription,
      });
    },
    showDeleteSuccess: () => {
      toast.success(t.projectDeletedSuccess);
    },
  };
}
