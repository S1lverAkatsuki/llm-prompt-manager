import { ref } from "vue";

const MAX_ERROR_LOGS = 100;

const errorLogs = ref<string[]>([]);

const normalizeError = (error: unknown): string => {
  if (typeof error === "string") return error;
  if (error instanceof Error) return error.message;

  try {
    return JSON.stringify(error);
  } catch {
    return String(error);
  }
};

export const useErrorLog = () => {
  const pushErrorLog = (scope: string, error: unknown) => {
    const timestamp = new Date().toLocaleString();
    const message = `[${timestamp}] ${scope}: ${normalizeError(error)}`;
    errorLogs.value = [message, ...errorLogs.value].slice(0, MAX_ERROR_LOGS);
    console.error(message, error);
  };

  const clearErrorLogs = () => {
    errorLogs.value = [];
  };

  return {
    errorLogs,
    pushErrorLog,
    clearErrorLogs,
  };
};
