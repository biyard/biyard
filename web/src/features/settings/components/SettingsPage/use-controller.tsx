import { useState } from "react";
import { State } from "@/types/state";
import { useNavigate } from "react-router-dom";
import { useAuth } from "@/contexts/AuthContext";
import { useWithdrawal } from "../../../auth/hooks/use-withdrawal";
import { useSettingsPageI18n } from "../../hooks/use-settings-page-i18n";

export class Controller {
  constructor(
    public showConfirmDialog: State<boolean>,
    public withdrawalMutation: ReturnType<typeof useWithdrawal>,
    public t: ReturnType<typeof useSettingsPageI18n>,
    public auth: ReturnType<typeof useAuth>,
    public navigate: ReturnType<typeof useNavigate>,
  ) {}

  get account() {
    return this.auth.account;
  }

  handleWithdrawal = async () => {
    try {
      await this.withdrawalMutation.mutateAsync();
      this.auth.setAccount(null);
      this.navigate("/signin");
    } catch (error) {
      console.error("Withdrawal failed:", error);
    }
  };
}

export function useController() {
  const t = useSettingsPageI18n();
  const navigate = useNavigate();
  const auth = useAuth();
  const showConfirmDialog = useState(false);
  const withdrawalMutation = useWithdrawal();

  return new Controller(
    new State(showConfirmDialog),
    withdrawalMutation,
    t,
    auth,
    navigate,
  );
}
