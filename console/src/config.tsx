const defaultApiUrl = window.location.origin;

export const config = {
  apiUrl: import.meta.env.VITE_API_URL || defaultApiUrl,
};
