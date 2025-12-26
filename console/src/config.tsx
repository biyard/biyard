const defaultApiUrl = window.location.origin;

export const config = {
  apiUrl: import.meta.env.VITE_API_BASE_URL || defaultApiUrl,
};
