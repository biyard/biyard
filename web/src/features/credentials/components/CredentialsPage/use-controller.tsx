import { useState } from "react";
import { State } from "@/types/state";
import { useListCredentials } from "../../hooks/use-list-credentials";
import { useCreateCredential } from "../../hooks/use-create-credential";
import { useRevokeCredential } from "../../hooks/use-revoke-credential";
import { useCredentialsPageI18n } from "../../hooks/use-credentials-page-i18n";

export class Controller {
  constructor(
    public showCreateDialog: State<boolean>,
    public newCredentialName: State<string>,
    public generatedKey: State<string | null>,
    public copiedKey: State<string | null>,
    public credentialsQuery: ReturnType<typeof useListCredentials>,
    public createMutation: ReturnType<typeof useCreateCredential>,
    public revokeMutation: ReturnType<typeof useRevokeCredential>,
    public t: ReturnType<typeof useCredentialsPageI18n>,
  ) {}

  get credentials() {
    return this.credentialsQuery.data;
  }

  get isLoading() {
    return this.credentialsQuery.isLoading;
  }

  handleCreateCredential = async () => {
    if (!this.newCredentialName.get().trim()) return;

    try {
      const response = await this.createMutation.mutateAsync({
        name: this.newCredentialName.get(),
      });

      this.generatedKey.set(response.api_key || null);
      this.newCredentialName.set("");
      this.showCreateDialog.set(false);
    } catch (error) {
      console.error("Failed to create credential:", error);
    }
  };

  handleRevokeCredential = async (credentialId: string) => {
    if (!confirm(this.t.confirmRevoke)) {
      return;
    }

    try {
      await this.revokeMutation.mutateAsync(credentialId);
    } catch (error) {
      console.error("Failed to revoke credential:", error);
    }
  };

  handleCopyKey = (key: string) => {
    navigator.clipboard.writeText(key);
    this.copiedKey.set(key);
    setTimeout(() => this.copiedKey.set(null), 2000);
  };

  maskKey = (key: string) => {
    if (key.length <= 16) return key;
    return `${key.substring(0, 12)}...${key.substring(key.length - 4)}`;
  };

  extractCredentialId = (pk: string) => {
    // pk format is "Credential(uuid)"
    const match = pk.match(/Credential\(([^)]+)\)/);
    return match ? match[1] : pk;
  };
}

export function useController() {
  const t = useCredentialsPageI18n();
  const showCreateDialog = useState(false);
  const newCredentialName = useState("");
  const generatedKey = useState<string | null>(null);
  const copiedKey = useState<string | null>(null);

  const credentialsQuery = useListCredentials();
  const createMutation = useCreateCredential();
  const revokeMutation = useRevokeCredential();

  return new Controller(
    new State(showCreateDialog),
    new State(newCredentialName),
    new State(generatedKey),
    new State(copiedKey),
    credentialsQuery,
    createMutation,
    revokeMutation,
    t,
  );
}
