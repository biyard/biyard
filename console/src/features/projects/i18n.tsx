import { useTranslation } from "react-i18next";

export const projects = {
  en: {
    title: "Projects",
  },
  ko: {
    title: "프로젝트",
  },
};

export interface ProjectsI18n {
  title: string;
}

export function useProjectsI18n(): ProjectsI18n {
  const { t } = useTranslation();

  return {
    title: t("projects.title"),
  };
}
