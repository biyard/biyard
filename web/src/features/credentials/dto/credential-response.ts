import { CredentialStatus } from "../types/credential-status";

export interface CredentialResponse {
  id: string;
  name: string;
  api_key_prefix: string;
  status: CredentialStatus;
  created_at: number;
  last_used_at?: number;
  api_key?: string; // Only present on creation
}
