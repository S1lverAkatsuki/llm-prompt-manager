<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { useSetting } from "@/composables/useSetting";
import { useAiConfig } from "@/composables/useAiConfig";
import { useErrorLog } from "@/composables/useErrorLog";
import { PROVIDERS } from "@/constants";
import type { ModelConfig } from "@/types";
import { X, Eye, EyeOff } from "lucide-vue-next";

const { dataPath, version } = useSetting();
const {
  isAiEnabled,
  modelConfig,
  loadAiConfig,
  toggleAi,
  saveModelConfig,
  clearAiConfig,
} = useAiConfig();
const { errorLogs, clearErrorLogs } = useErrorLog();

const dialogRef = ref<HTMLDialogElement | null>(null);

const show = () => {
  loadAiConfig();
  dialogRef.value?.showModal();
};

const apiKey = ref("");
const selectedProvider = ref("");
const selectedModel = ref("");
const customModel = ref("");
const customBaseUrl = ref("");
const showKey = ref(false);

watch(
  modelConfig,
  config => {
    if (config) {
      apiKey.value = config.api_key;
      selectedProvider.value = config.provider;
      if (!PROVIDERS.some(p => p.value === selectedProvider.value)) {
        selectedProvider.value = "";
      }
      selectedModel.value = config.model;
      if (
        selectedModel.value &&
        !currentProvider.value?.models.includes(selectedModel.value)
      ) {
        selectedModel.value = "";
      }
      customModel.value = config.model;
      customBaseUrl.value = config.base_url;
    }
  },
  { immediate: true }
);

const currentProvider = computed(() =>
  PROVIDERS.find(p => p.value === selectedProvider.value)
);

const isCustom = computed(() => selectedProvider.value === "custom");

const isApiKeyEmpty = computed(() => !apiKey.value);
const isProviderEmpty = computed(() => !selectedProvider.value);
const isModelEmpty = computed(
  () => !(isCustom.value ? customModel.value : selectedModel.value)
);
const isBaseUrlEmpty = computed(() => isCustom.value && !customBaseUrl.value);

watch(selectedProvider, (_, oldVal) => {
  if (!oldVal) return;
  selectedModel.value = "";
  customModel.value = "";
  customBaseUrl.value = "";
});

const currentModel = computed(() =>
  isCustom.value ? customModel.value : selectedModel.value
);

const currentUrl = computed(() =>
  isCustom.value
    ? customBaseUrl.value
    : (currentProvider.value?.defaultUrl ?? "")
);

const canEnableAi = computed(() => {
  if (!apiKey.value) return false;
  if (!selectedProvider.value) return false;
  if (!currentModel.value) return false;
  if (isCustom.value && !customBaseUrl.value) return false;
  return true;
});

const hasChanged = computed(() => {
  const saved = modelConfig.value;

  if (!apiKey.value) return false;
  if (!selectedProvider.value) return false;
  if (!currentModel.value) return false;
  if (isCustom.value && !customBaseUrl.value) return false;

  if (!saved) return true;

  return (
    apiKey.value !== saved.api_key ||
    selectedProvider.value !== saved.provider ||
    currentModel.value !== saved.model ||
    currentUrl.value !== saved.base_url
  );
});

const handleSaveModelConfig = async () => {
  const config: ModelConfig = {
    api_key: apiKey.value,
    provider: selectedProvider.value,
    model: currentModel.value,
    base_url: currentUrl.value,
  };
  await saveModelConfig(config);
};

const handleAiToggle = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const checked = target.checked;

  if (!checked) {
    await toggleAi(false);
    return;
  }

  if (!canEnableAi.value) {
    target.checked = false;
    return;
  }

  await toggleAi(true);
};

const handleClearAiConfig = async () => {
  await clearAiConfig();
  apiKey.value = "";
  selectedProvider.value = "";
  selectedModel.value = "";
  customModel.value = "";
  customBaseUrl.value = "";
  showKey.value = false;
};

defineExpose({ show });
</script>

<template>
  <dialog ref="dialogRef" id="setting" class="modal transition-none!">
    <div class="modal-box w-[55vw] max-w-3xl h-[90vh] p-0 flex flex-col">
      <div
        class="flex items-center justify-between border-b border-base-300 px-6 py-4"
      >
        <h3 class="text-lg font-semibold text-base-content">设置</h3>
        <form method="dialog">
          <button class="btn btn-sm btn-ghost p-1">
            <X class="w-5" />
          </button>
        </form>
      </div>
      <div
        class="flex-1 overflow-y-auto p-4 space-y-6 [scrollbar-gutter:stable_both-edges]"
      >
        <div class="flex items-center justify-between">
          <div>
            <p class="text-base font-medium">启用 AI 功能</p>
            <p class="text-xs text-base-content/50">
              开启后可使用 AI 生成提示词
            </p>
          </div>
          <input
            type="checkbox"
            class="toggle toggle-primary"
            :checked="isAiEnabled"
            @change="handleAiToggle"
          />
        </div>
        <fieldset
          v-if="isAiEnabled"
          class="fieldset p-4 border border-base-300 rounded-box"
        >
          <legend class="fieldset-legend text-base font-medium">
            模型配置
          </legend>
          <p class="text-xs text-base-content/50 -mt-2">
            填写的 API KEY 只会保存在本地的配置文件中
            <br />
            <a
              href="#"
              class="link link-error"
              @click.prevent="handleClearAiConfig"
            >
              清除所有 AI 配置信息
            </a>
          </p>

          <label class="fieldset-label text-base-content mt-2">API KEY</label>
          <label
            class="input input-sm w-full flex items-center gap-2"
            :class="{ 'input-error': isApiKeyEmpty }"
          >
            <input
              v-model="apiKey"
              type="text"
              class="grow"
              :class="{ 'password-mask': !showKey }"
              placeholder="输入你的 API KEY"
            />
            <button
              class="btn btn-ghost btn-xs h-5 w-5 p-0"
              @click="showKey = !showKey"
            >
              <EyeOff v-if="showKey" class="w-3.5" />
              <Eye v-else class="w-3.5" />
            </button>
          </label>
          <label class="fieldset-label text-base-content mt-2">提供商</label>
          <select
            v-model="selectedProvider"
            class="select select-sm w-full"
            :class="{ 'select-error': isProviderEmpty }"
          >
            <option disabled value="">请选择提供商</option>
            <option
              v-for="prov in PROVIDERS"
              :key="prov.value"
              :value="prov.value"
            >
              {{ prov.label }}
            </option>
          </select>
          <label class="fieldset-label text-base-content mt-2">模型</label>
          <select
            v-if="!isCustom"
            v-model="selectedModel"
            class="select select-sm w-full"
            :class="{ 'select-error': isModelEmpty }"
          >
            <option disabled value="">请选择模型</option>
            <option
              v-for="model in currentProvider?.models ?? []"
              :key="model"
              :value="model"
            >
              {{ model }}
            </option>
          </select>
          <input
            v-else
            v-model="customModel"
            class="input input-sm w-full"
            :class="{ 'input-error': isModelEmpty }"
            placeholder="输入模型名称"
          />
          <div v-show="isCustom" class="mt-2">
            <label class="fieldset-label text-base-content mb-1"
              >请求 URL</label
            >
            <input
              v-model="customBaseUrl"
              class="input input-sm w-full"
              :class="{ 'input-error': isBaseUrlEmpty }"
              placeholder="https://api.example.com/v1"
            />
          </div>
          <button
            v-if="hasChanged"
            class="btn btn-primary btn-sm mt-4"
            @click="handleSaveModelConfig"
          >
            保存
          </button>
        </fieldset>
        <div>
          <p class="text-sm font-medium">配置文件位置</p>
          <p class="text-sm text-base-content/50 mt-2 mb-2">
            跟随 Tauri 默认应用配置，很显然没有改的必要
          </p>
          <a
            class="link link-info w-full whitespace-normal break-all"
            title="用资源管理器打开"
            @click="revealItemInDir(dataPath!)"
            >{{ dataPath }}</a
          >
        </div>
        <div>
          <div class="flex items-center justify-between">
            <p class="text-sm font-medium">错误日志</p>
            <button class="btn btn-ghost btn-xs" @click="clearErrorLogs">
              清空
            </button>
          </div>
          <textarea
            class="textarea textarea-sm w-full mt-2 h-40 resize-none overflow-y-auto font-mono"
            :value="errorLogs.join('\n\n')"
            readonly
            placeholder="当前没有错误日志"
          />
        </div>
        <div>
          <p class="text-sm font-medium">关于</p>
          <div class="card w-full bg-base-200/50 card-sm shadow mt-2">
            <div class="card-body">
              <div class="flex flex-row gap-2 items-center">
                <h2 class="card-title">LLM-Prompt-Manager</h2>
                <span class="inline ml-auto text-sm text-base-content">{{
                  version
                }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </dialog>
</template>

<style scoped>
.modal {
  z-index: 998 !important;
}

.password-mask {
  -webkit-text-security: disc;
}
</style>
