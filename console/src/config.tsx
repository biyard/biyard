const defaultApiBaseUrl = window.location.origin;

export const config = {
  apiBaseUrl: import.meta.env.VITE_API_BASE_URL || defaultApiBaseUrl,
};
