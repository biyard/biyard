import { useState } from "react";
import { State } from "@/types/state";
import { useNavigate } from "react-router-dom";
import { useAuth } from "@/contexts/AuthContext";
import { useSignin } from "../../hooks/use-signin";
import { useSignInPageI18n } from "../../hooks/use-sign-in-page-i18n";

export class Controller {
  constructor(
    public email: State<string>,
    public password: State<string>,
    public error: State<string>,
    public signinMutation: ReturnType<typeof useSignin>,
    public t: ReturnType<typeof useSignInPageI18n>,
    public auth: ReturnType<typeof useAuth>,
    public navigate: ReturnType<typeof useNavigate>,
  ) {}

  handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    this.error.set("");

    if (!this.email.get() || !this.password.get()) {
      this.error.set(this.t.emailRequired);
      return;
    }

    try {
      const account = await this.signinMutation.mutateAsync({
        email: this.email.get(),
        password: this.password.get(),
      });
      this.auth.setAccount(account);
      this.navigate("/dashboard");
    } catch (_err) {
      this.error.set(this.t.signInError);
    }
  };
}

export function useController() {
  const t = useSignInPageI18n();
  const navigate = useNavigate();
  const auth = useAuth();
  const email = useState("");
  const password = useState("");
  const error = useState("");

  const signinMutation = useSignin();

  return new Controller(
    new State(email),
    new State(password),
    new State(error),
    signinMutation,
    t,
    auth,
    navigate,
  );
}
