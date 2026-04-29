import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

export function useTheme() {
  const isInDarkMode = ref<boolean | null>(null);

  invoke<boolean>("get_dark_mode")
    .then(x => isInDarkMode.value = x)
    .catch(e => console.error(e));

  watch(isInDarkMode, (value, oldValue) => {
    if (value === null) return;

    const theme = value ? "dark" : "light";
    document.documentElement.setAttribute("data-theme", theme);

    if (oldValue === null) return;

    invoke("set_dark_mode", { newMode: value })
      .catch(e => console.error(e));
  });

  return { isInDarkMode };
}
