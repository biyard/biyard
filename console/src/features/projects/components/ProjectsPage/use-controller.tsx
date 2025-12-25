import { useState } from "react";
import { State } from "@/types/state";
import { useListProjects } from "../../hooks/use-list-projects";
import { useCreateProject } from "../../hooks/use-create-project";
import { useDeleteProject } from "../../hooks/use-delete-project";
import { useProjectsPageI18n } from "./i18n";

export class Controller {
  constructor(
    public showCreateDialog: State<boolean>,
    public projectName: State<string>,
    public description: State<string>,
    public monthlyTokenSupply: State<string>,
    public symbol: State<string>,
    public decimals: State<string>,
    public error: State<string>,
    public projectsQuery: ReturnType<typeof useListProjects>,
    public createMutation: ReturnType<typeof useCreateProject>,
    public deleteMutation: ReturnType<typeof useDeleteProject>,
    public t: ReturnType<typeof useProjectsPageI18n>,
  ) {}

  get projects() {
    return this.projectsQuery.data;
  }

  get isLoading() {
    return this.projectsQuery.isLoading;
  }

  handleCreateProject = async () => {
    this.error.set("");

    if (!this.projectName.get().trim()) {
      this.error.set(this.t.projectNameRequired);
      return;
    }

    if (!this.symbol.get().trim()) {
      this.error.set(this.t.symbolRequired);
      return;
    }

    const tokenSupply = parseFloat(this.monthlyTokenSupply.get());

    // Allow zero for manual provisioning, but must be a valid non-negative number
    if (isNaN(tokenSupply) || tokenSupply < 0) {
      this.error.set(this.t.tokenSupplyMustBeNonNegative);
      return;
    }

    const decimalsValue = this.decimals.get().trim();
    const decimals = decimalsValue ? parseInt(decimalsValue) : undefined;
    if (decimalsValue && (isNaN(decimals!) || decimals! < 0 || decimals! > 18)) {
      this.error.set(this.t.decimalsMustBeValid);
      return;
    }

    try {
      await this.createMutation.mutateAsync({
        name: this.projectName.get(),
        description: this.description.get() || undefined,
        monthly_token_supply: tokenSupply,
        symbol: this.symbol.get(),
        decimals,
      });

      // Clear form and close dialog on success
      this.projectName.set("");
      this.description.set("");
      this.monthlyTokenSupply.set("");
      this.symbol.set("");
      this.decimals.set("");
      this.showCreateDialog.set(false);
    } catch (_err) {
      // Error toast is already shown by the mutation hook
      // Just keep the dialog open so user can retry
    }
  };

  handleDeleteProject = async (projectId: string) => {
    if (!confirm(this.t.confirmDelete)) {
      return;
    }

    try {
      await this.deleteMutation.mutateAsync(projectId);
      // Success toast is already shown by the mutation hook
    } catch (_err) {
      // Error toast is already shown by the mutation hook
      // No need to log to console as user sees the toast
    }
  };
}

export function useController() {
  const t = useProjectsPageI18n();
  const showCreateDialog = useState(false);
  const projectName = useState("");
  const description = useState("");
  const monthlyTokenSupply = useState("");
  const symbol = useState("");
  const decimals = useState("");
  const error = useState("");

  const projectsQuery = useListProjects();
  const createMutation = useCreateProject();
  const deleteMutation = useDeleteProject();

  return new Controller(
    new State(showCreateDialog),
    new State(projectName),
    new State(description),
    new State(monthlyTokenSupply),
    new State(symbol),
    new State(decimals),
    new State(error),
    projectsQuery,
    createMutation,
    deleteMutation,
    t,
  );
}
