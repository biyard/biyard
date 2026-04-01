export interface Account {
  pk: string;
  name: string;
  email: string;
  created_at: number;
}

export interface SignupRequest {
  name: string;
  email: string;
  hashed_password: string;
}

export interface SigninRequest {
  email: string;
  password: string;
}

export interface AccountResponse {
  pk: string;
  name: string;
  email: string;
  created_at: number;
}
