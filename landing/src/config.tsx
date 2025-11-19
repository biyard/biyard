let defaultApiBaseUrl = window.location.origin;

if (defaultApiBaseUrl.includes("localhost")) {
  defaultApiBaseUrl = "http://localhost:3000";
}

export const config = {
  apiBaseUrl: import.meta.env.VITE_API_BASE_URL || defaultApiBaseUrl,
  logLevel: import.meta.env.VITE_LOG_LEVEL || "debug",
};
