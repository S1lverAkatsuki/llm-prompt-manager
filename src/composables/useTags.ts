import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export const useTags = () => {
  const tags = ref<string[]>([]);

  const refresh = async () => {
    try {
      tags.value = await invoke<string[]>("get_all_tags");
    } catch (e) {
      console.error(e);
    }
  };

  const addTag = async (tag: string) => {
    invoke("add_tag", { tag: tag }).catch(console.error)
  };

  const removeTag = async (tag: string) => {
    invoke("remove_tag", { tag: tag }).catch(console.error)
  };

  refresh();

  return { tags, addTag, removeTag, refresh };
};
