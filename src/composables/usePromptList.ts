import { ref, onBeforeUnmount } from "vue";
import { Prompt } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

export function usePromptList() {
  const copiedId = ref<string | null>(null);
  const copyTimeout = ref<NodeJS.Timeout | null>(null);

  const expandedId = ref<string | null>(null);

  const items = ref<Prompt[] | null>(null);

  invoke<Prompt[]>("read")
    .then(data => items.value = data)
    .catch(e => {
      items.value = [];
      console.error(e);
    });

  const toggleExpand = (id: string) => {
    expandedId.value = expandedId.value === id ? null : id;
  };

  const handleCopy = async (id: string, content: string) => {
    try {
      await writeText(content);
      copiedId.value = id;

      if (copyTimeout.value) {
        clearTimeout(copyTimeout.value);
      }

      const RESET_MILLISECOND = 2000;
      copyTimeout.value = setTimeout(() => {
        copiedId.value = null;
      }, RESET_MILLISECOND);
    } catch (err) {
    }
  };

  onBeforeUnmount(() => {
    if (copyTimeout.value) {
      clearTimeout(copyTimeout.value);
    }
  });

  return { items, expandedId, toggleExpand, copiedId, handleCopy };
}
