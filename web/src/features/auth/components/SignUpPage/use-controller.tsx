import { useState, useEffect } from "react";
import { State } from "@/types/state";
import { useNavigate } from "react-router-dom";
import { useAuth } from "@/contexts/AuthContext";
import { useSignup } from "../../hooks/use-signup";
import { useSignUpPageI18n } from "./i18n";

export class Controller {
  constructor(
    public name: State<string>,
    public email: State<string>,
    public password: State<string>,
    public confirmPassword: State<string>,
    public error: State<string>,
    public signupMutation: ReturnType<typeof useSignup>,
    public t: ReturnType<typeof useSignUpPageI18n>,
    public auth: ReturnType<typeof useAuth>,
    public navigate: ReturnType<typeof useNavigate>,
  ) {}

  handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    this.error.set("");

    if (!this.name.get() || !this.email.get() || !this.password.get()) {
      this.error.set(this.t.nameRequired);
      return;
    }

    if (this.password.get() !== this.confirmPassword.get()) {
      this.error.set(this.t.passwordMismatch);
      return;
    }

    try {
      const account = await this.signupMutation.mutateAsync({
        name: this.name.get(),
        email: this.email.get(),
        hashed_password: this.password.get(), // Backend will hash it
      });
      this.auth.setAccount(account);
      this.navigate("/dashboard");
    } catch (_err) {
      this.error.set(this.t.signUpError);
    }
  };
}

export function useController() {
  const t = useSignUpPageI18n();
  const navigate = useNavigate();
  const auth = useAuth();
  const name = useState("");
  const email = useState("");
  const password = useState("");
  const confirmPassword = useState("");
  const error = useState("");

  const signupMutation = useSignup();

  // Redirect to dashboard if already authenticated
  useEffect(() => {
    if (auth.isAuthenticated && !auth.isLoading) {
      navigate("/dashboard", { replace: true });
    }
  }, [auth.isAuthenticated, auth.isLoading, navigate]);

  return new Controller(
    new State(name),
    new State(email),
    new State(password),
    new State(confirmPassword),
    new State(error),
    signupMutation,
    t,
    auth,
    navigate,
  );
}
