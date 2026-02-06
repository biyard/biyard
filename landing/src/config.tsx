const defaultApiBaseUrl = window.location.origin;

export const config = {
  apiBaseUrl: import.meta.env.VITE_API_BASE_URL || defaultApiBaseUrl,
  consoleUrl: import.meta.env.VITE_CONSOLE_URL || "https://console.biyard.co",
  logLevel: import.meta.env.VITE_LOG_LEVEL || "debug",
};
