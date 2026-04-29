<script setup lang="ts">
import { ref } from "vue";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { useSetting } from "@/composables/useSetting";

const { data_path, version } = useSetting();

const dialogRef = ref<HTMLDialogElement | null>(null);

const show = () => {
  dialogRef.value?.showModal();
};

defineExpose({ show });
</script>

<template>
  <dialog
    ref="dialogRef"
    id="setting"
    class="modal transition-none!"
  >
    <div class="modal-box w-[50vw] max-w-3xl h-[90vh] p-0 flex flex-col">
      <div
        class="flex items-center justify-between border-b border-base-300 px-6 py-4"
      >
        <h3 class="text-lg font-semibold text-base-content">设置</h3>
        <form method="dialog">
          <button class="btn btn-sm btn-circle btn-ghost">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              class="w-4 h-4"
              fill="currentColor"
            >
              <path
                d="M19,6.41,17.59,5,12,10.59,6.41,5,5,6.41,10.59,12,5,17.59,6.41,19,12,13.41,17.59,19,19,17.59,13.41,12Z"
              />
            </svg>
          </button>
        </form>
      </div>
      <div class="flex-1 overflow-y-auto p-6 py-4 space-y-6">
        <div>
          <p class="text-sm font-medium">配置文件位置</p>
          <p class="text-sm text-base-content/50 mt-2 mb-2">
            跟随 Tauri 默认应用配置，很显然没有改的必要
          </p>
          <a
            class="link link-info w-full whitespace-normal break-all"
            title="用资源管理器打开"
            @click="revealItemInDir(data_path!)"
            >{{ data_path }}</a
          >
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
              <p class="text-sm text-base-content/50">
                银晓洗脑机器人用的邪恶工具 👎🤖
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </dialog>
</template>
