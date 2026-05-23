import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useErrorLog } from "@/composables/useErrorLog";

export const useTags = () => {
  const { pushErrorLog } = useErrorLog();
  const tags = ref<string[]>([]);

  const refresh = async () => {
    try {
      tags.value = await invoke<string[]>("get_all_tags");
    } catch (e) {
      pushErrorLog("获取标签列表失败", e);
    }
  };

  const addTag = async (tag: string) => {
    invoke("add_tag", { tag: tag }).catch(e => pushErrorLog("添加标签失败", e));
  };

  const removeTag = async (tag: string) => {
    invoke("remove_tag", { tag: tag }).catch(e => pushErrorLog("删除标签失败", e));
  };

  refresh();

  return { tags, addTag, removeTag, refresh };
};
