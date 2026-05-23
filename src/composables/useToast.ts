import { ref } from "vue";

type ToastLevel = "info" | "success" | "warning" | "error";

export interface ToastItem {
  id: number;
  message: string;
  level: ToastLevel;
}

const toasts = ref<ToastItem[]>([]);
let nextToastId = 1;

export const useToast = () => {
  const removeToast = (id: number) => {
    toasts.value = toasts.value.filter(toast => toast.id !== id);
  };

  const pushToast = (
    message: string,
    level: ToastLevel = "info",
    duration = 3000
  ) => {
    const id = nextToastId++;
    toasts.value = [...toasts.value, { id, message, level }];

    if (duration > 0) {
      window.setTimeout(() => {
        removeToast(id);
      }, duration);
    }
  };

  return {
    toasts,
    pushToast,
    removeToast,
  };
};
