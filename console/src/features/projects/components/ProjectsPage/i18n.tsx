import { useTranslation } from "react-i18next";
import { useAppI18n } from "@/i18n/locales/app";
import { useCommonI18n } from "@/i18n/locales/common";

export const ProjectsPage = {
  en: {
    description: "Projects",
    createNew: "Create New Project",
    noProjects: "No projects yet",
    noProjectsDescription: "Create your first project to get started",
    projectId: "Project ID",
    projectName: "Project Name",
    projectDescription: "Description",
    monthlyTokenSupply: "Monthly Token Supply",
    tokenSymbol: "Token Symbol",
    tokenDecimals: "Token Decimals",
    status: "Status",
    createdAt: "Created",
    actions: "Actions",
    view: "View",
    delete: "Delete",
    confirmDelete: "Are you sure you want to delete this project?",
    createProject: "Create Project",
    enterProjectName: "Enter project name",
    enterDescription: "Enter description (optional)",
    enterTokenSupply: "Enter monthly token supply (0 for manual provisioning)",
    enterSymbol: "Enter token symbol (e.g., BTC, ETH)",
    enterDecimals: "Enter decimals (0-18, default: 0)",
    creating: "Creating...",
    projectNameRequired: "Project name is required",
    symbolRequired: "Token symbol is required",
    tokenSupplyMustBeNonNegative: "Token supply must be zero or a positive number",
    decimalsMustBeValid: "Decimals must be between 0 and 18",
    errorLoadingProjects: "Failed to load projects",
    errorLoadingProjectsDescription: "Please try again later",
    errorCreatingProject: "Failed to create project",
    errorCreatingProjectDescription: "Please check your input and try again",
    errorDeletingProject: "Failed to delete project",
    errorDeletingProjectDescription: "Please try again later",
    projectCreatedSuccess: "Project created successfully",
    projectDeletedSuccess: "Project deleted successfully",
  },
  ko: {
    description: "프로젝트",
    createNew: "새 프로젝트 생성",
    noProjects: "프로젝트가 없습니다",
    noProjectsDescription: "첫 번째 프로젝트를 생성하여 시작하세요",
    projectId: "프로젝트 ID",
    projectName: "프로젝트 이름",
    projectDescription: "설명",
    monthlyTokenSupply: "월별 토큰 공급량",
    tokenSymbol: "토큰 심볼",
    tokenDecimals: "토큰 소수점",
    status: "상태",
    createdAt: "생성일",
    actions: "작업",
    view: "보기",
    delete: "삭제",
    confirmDelete: "이 프로젝트를 삭제하시겠습니까?",
    createProject: "프로젝트 생성",
    enterProjectName: "프로젝트 이름 입력",
    enterDescription: "설명 입력 (선택사항)",
    enterTokenSupply: "월별 토큰 공급량 입력 (수동 프로비저닝은 0)",
    enterSymbol: "토큰 심볼 입력 (예: BTC, ETH)",
    enterDecimals: "소수점 입력 (0-18, 기본값: 0)",
    creating: "생성 중...",
    projectNameRequired: "프로젝트 이름은 필수입니다",
    symbolRequired: "토큰 심볼은 필수입니다",
    tokenSupplyMustBeNonNegative: "토큰 공급량은 0 또는 양수여야 합니다",
    decimalsMustBeValid: "소수점은 0에서 18 사이여야 합니다",
    errorLoadingProjects: "프로젝트를 불러오는데 실패했습니다",
    errorLoadingProjectsDescription: "나중에 다시 시도해주세요",
    errorCreatingProject: "프로젝트 생성에 실패했습니다",
    errorCreatingProjectDescription: "입력 내용을 확인하고 다시 시도해주세요",
    errorDeletingProject: "프로젝트 삭제에 실패했습니다",
    errorDeletingProjectDescription: "나중에 다시 시도해주세요",
    projectCreatedSuccess: "프로젝트가 성공적으로 생성되었습니다",
    projectDeletedSuccess: "프로젝트가 성공적으로 삭제되었습니다",
  },
};

export interface ProjectsPageI18n {
  // From app
  title: string;
  tagline: string;

  // From common
  loading: string;
  cancel: string;
  close: string;
  save: string;
  delete: string;
  confirm: string;
  actions: string;

  // ProjectsPage specific
  description: string;
  createNew: string;
  noProjects: string;
  noProjectsDescription: string;
  projectId: string;
  projectName: string;
  projectDescription: string;
  monthlyTokenSupply: string;
  tokenSymbol: string;
  tokenDecimals: string;
  status: string;
  createdAt: string;
  view: string;
  confirmDelete: string;
  createProject: string;
  enterProjectName: string;
  enterDescription: string;
  enterTokenSupply: string;
  enterSymbol: string;
  enterDecimals: string;
  creating: string;
  projectNameRequired: string;
  symbolRequired: string;
  tokenSupplyMustBeNonNegative: string;
  decimalsMustBeValid: string;
  errorLoadingProjects: string;
  errorLoadingProjectsDescription: string;
  errorCreatingProject: string;
  errorCreatingProjectDescription: string;
  errorDeletingProject: string;
  errorDeletingProjectDescription: string;
  projectCreatedSuccess: string;
  projectDeletedSuccess: string;
}

export function useProjectsPageI18n(): ProjectsPageI18n {
  const { t } = useTranslation();
  const app = useAppI18n();
  const common = useCommonI18n();

  return {
    // From app
    title: app.title,
    tagline: app.tagline,

    // From common
    loading: common.loading,
    cancel: common.cancel,
    close: common.close,
    save: common.save,
    delete: common.delete,
    confirm: common.confirm,
    actions: common.actions,

    // ProjectsPage specific
    description: t("ProjectsPage.description"),
    createNew: t("ProjectsPage.createNew"),
    noProjects: t("ProjectsPage.noProjects"),
    noProjectsDescription: t("ProjectsPage.noProjectsDescription"),
    projectId: t("ProjectsPage.projectId"),
    projectName: t("ProjectsPage.projectName"),
    projectDescription: t("ProjectsPage.projectDescription"),
    monthlyTokenSupply: t("ProjectsPage.monthlyTokenSupply"),
    tokenSymbol: t("ProjectsPage.tokenSymbol"),
    tokenDecimals: t("ProjectsPage.tokenDecimals"),
    status: t("ProjectsPage.status"),
    createdAt: t("ProjectsPage.createdAt"),
    view: t("ProjectsPage.view"),
    confirmDelete: t("ProjectsPage.confirmDelete"),
    createProject: t("ProjectsPage.createProject"),
    enterProjectName: t("ProjectsPage.enterProjectName"),
    enterDescription: t("ProjectsPage.enterDescription"),
    enterTokenSupply: t("ProjectsPage.enterTokenSupply"),
    enterSymbol: t("ProjectsPage.enterSymbol"),
    enterDecimals: t("ProjectsPage.enterDecimals"),
    creating: t("ProjectsPage.creating"),
    projectNameRequired: t("ProjectsPage.projectNameRequired"),
    symbolRequired: t("ProjectsPage.symbolRequired"),
    tokenSupplyMustBeNonNegative: t("ProjectsPage.tokenSupplyMustBeNonNegative"),
    decimalsMustBeValid: t("ProjectsPage.decimalsMustBeValid"),
    errorLoadingProjects: t("ProjectsPage.errorLoadingProjects"),
    errorLoadingProjectsDescription: t("ProjectsPage.errorLoadingProjectsDescription"),
    errorCreatingProject: t("ProjectsPage.errorCreatingProject"),
    errorCreatingProjectDescription: t("ProjectsPage.errorCreatingProjectDescription"),
    errorDeletingProject: t("ProjectsPage.errorDeletingProject"),
    errorDeletingProjectDescription: t("ProjectsPage.errorDeletingProjectDescription"),
    projectCreatedSuccess: t("ProjectsPage.projectCreatedSuccess"),
    projectDeletedSuccess: t("ProjectsPage.projectDeletedSuccess"),
  };
}
