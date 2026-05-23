import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useErrorLog } from "@/composables/useErrorLog";

export const useTheme = () => {
  const { pushErrorLog } = useErrorLog();
  const isInDarkMode = ref<boolean | null>(null);

  invoke<boolean>("get_dark_mode")
    .then(x => (isInDarkMode.value = x))
    .catch(e => pushErrorLog("获取暗色模式设置失败", e));

  watch(isInDarkMode, (value, oldValue) => {
    if (value === null) return;

    const theme = value ? "dark" : "light";
    document.documentElement.setAttribute("data-theme", theme);

    if (oldValue === null) return;

    invoke("set_dark_mode", { newMode: value }).catch(e => pushErrorLog("设置暗色模式失败", e));
  });

  return { isInDarkMode };
};
