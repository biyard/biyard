export class BiyardError extends Error {
  constructor(message: string, public readonly cause?: unknown) {
    super(message);
    this.name = "BiyardError";
  }
}

export class WalletNotInstalledError extends BiyardError {
  constructor() {
    super("No wallet extension found. Please install Kaia Wallet or MetaMask.");
    this.name = "WalletNotInstalledError";
  }
}

export class UserRejectedError extends BiyardError {
  constructor(action: string) {
    super(`User rejected ${action}.`);
    this.name = "UserRejectedError";
  }
}

export class UnsupportedChainError extends BiyardError {
  constructor(chainId: number) {
    super(`Unsupported chain: ${chainId}`);
    this.name = "UnsupportedChainError";
  }
}

export class ProxyRequestError extends BiyardError {
  constructor(
    public readonly status: number,
    public readonly body: string,
    public readonly path: string,
  ) {
    super(`Partner proxy returned ${status} for ${path}: ${body}`);
    this.name = "ProxyRequestError";
  }
}
