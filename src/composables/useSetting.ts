import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export function useSetting() {
  const data_path = ref<string>("");
  const version = ref<string>("");

  invoke<string>("get_data_path")
    .then(path => (data_path.value = path))
    .catch(e => console.error(e));
  invoke<string>("get_version")
    .then(x => (version.value = x))
    .catch(e => console.error(e));

  return { data_path, version };
}
