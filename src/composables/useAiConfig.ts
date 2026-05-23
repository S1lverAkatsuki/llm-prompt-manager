import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ModelConfig } from "@/types";
import { useErrorLog } from "@/composables/useErrorLog";

const isAiEnabled = ref<boolean>(false);
const modelConfig = ref<ModelConfig | null>(null);
const loaded = ref<boolean>(false);

export const useAiConfig = () => {
  const { pushErrorLog } = useErrorLog();

  const loadAiConfig = async () => {
    if (loaded.value) return;
    try {
      isAiEnabled.value = await invoke<boolean>("get_ai_enabled");
      modelConfig.value = await invoke<ModelConfig | null>("get_model_config");
    } catch (e) {
      pushErrorLog("加载 AI 配置失败", e);
    }
    loaded.value = true;
  };

  const toggleAi = async (enabled: boolean) => {
    try {
      await invoke("set_ai_enabled", { enabled });
      isAiEnabled.value = enabled;
    } catch (e) {
      pushErrorLog("切换 AI 功能失败", e);
    }
  };

  const saveModelConfig = async (config: ModelConfig) => {
    try {
      await invoke("set_model_config", { config });
      modelConfig.value = config;
    } catch (e) {
      pushErrorLog("保存模型配置失败", e);
    }
  };

  const clearAiConfig = async () => {
    try {
      await invoke("clear_ai_config");
      isAiEnabled.value = false;
      modelConfig.value = null;
    } catch (e) {
      pushErrorLog("清除 AI 配置失败", e);
    }
  };

  return {
    isAiEnabled,
    modelConfig,
    loadAiConfig,
    toggleAi,
    saveModelConfig,
    clearAiConfig,
  };
};
