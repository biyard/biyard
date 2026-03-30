import { useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useGetProject } from "../../hooks/use-get-project";
import { useGetToken } from "../../../tokens/hooks/use-get-token";
import { useListPointTransactions } from "../../../points/hooks/use-list-point-transactions";
import { useProjectDetailI18n } from "./i18n";

export type TabType = "overview" | "treasury" | "tokens" | "points" | "users" | "audit" | "settings";

export function useController() {
  const { projectId } = useParams<{ projectId: string }>();
  const navigate = useNavigate();
  const t = useProjectDetailI18n();

  const { data: project, isLoading, isError } = useGetProject(projectId || "");
  const {
    data: token,
    isLoading: isLoadingToken,
  } = useGetToken(projectId || "");
  const {
    data: pointTransactionsData,
    isLoading: isLoadingPointTransactions,
  } = useListPointTransactions(projectId || "");

  const [activeTab, setActiveTab] = useState<TabType>("overview");

  const handleBack = () => {
    navigate("/projects");
  };

  return {
    t,
    projectId,
    project,
    isLoading,
    isError,
    activeTab,
    setActiveTab,
    handleBack,
    token,
    isLoadingToken,
    pointTransactions: pointTransactionsData?.transactions || [],
    isLoadingPointTransactions,
  };
}
