import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useErrorLog } from "@/composables/useErrorLog";

export const useSetting = () => {
  const { pushErrorLog } = useErrorLog();
  const dataPath = ref<string>("");
  const version = ref<string>("");

  invoke<string>("get_data_path")
    .then(path => (dataPath.value = path))
    .catch(e => pushErrorLog("获取数据路径失败", e));
  invoke<string>("get_version")
    .then(x => (version.value = x))
    .catch(e => pushErrorLog("获取版本号失败", e));

  return { dataPath, version };
};
