import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export const useSetting = () => {
  const dataPath = ref<string>("");
  const version = ref<string>("");

  invoke<string>("get_data_path")
    .then(path => (dataPath.value = path))
    .catch(e => console.error(e));
  invoke<string>("get_version")
    .then(x => (version.value = x))
    .catch(e => console.error(e));

  return { dataPath, version };
};
