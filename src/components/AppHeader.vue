<script setup lang="ts">
import { Funnel, Moon, Plus, Settings, Sun } from "lucide-vue-next";
import { useFilterStore } from "@/composables/useFilterStore";
import { ref } from "vue";
import SelectedTagDialog from "./SelectedTagDialog.vue";

defineProps<{
  isInDarkMode: boolean | null;
}>();

defineEmits<{
  toggleDark: [];
  openSetting: [];
  create: [];
}>();

const tagsSelectorDialogRef = ref<HTMLDialogElement | null>(null);

const openTagSelector = () => tagsSelectorDialogRef.value?.show();

const { searchedTitle, selectedTags } = useFilterStore();
</script>

<template>
  <div class="w-full h-26 bg-base-100 border-base-300 border-b">
    <div class="h-full flex flex-col p-4 gap-4">
      <div class="w-full flex flex-row items-center gap-2 flex-nowrap">
        <a
          href="https://github.com/S1lverAkatsuki/llm-prompt-manager"
          target="_blank"
          rel="noopener noreferrer"
          class="text-primary font-bold text-xl shrink-0 hover:brightness-80 cursor-pointer"
          title="前往Github查看"
        >
          LLM-Prompt-Manager
        </a>
        <label
          class="swap swap-rotate btn btn-ghost btn-sm text-base-content hover:bg-base-300"
          style="padding-inline: 0.25rem"
        >
          <input
            type="checkbox"
            class="theme-controller"
            value="dark"
            :checked="isInDarkMode === true"
            @change="$emit('toggleDark')"
            :disabled="isInDarkMode === null"
          />

          <Sun class="swap-on w-5" />

          <Moon class="swap-off w-5" />
        </label>
        <button
          class="btn btn-ghost btn-sm text-base-content hover:bg-base-300"
          style="padding-inline: 0.25rem"
          @click="$emit('openSetting')"
        >
          <Settings class="w-5" />
        </button>
        <button
          class="btn ml-auto btn-primary whitespace-nowrap pl-5 pr-5"
          @click="$emit('create')"
        >
          <Plus class="w-5" />
          添加新项
        </button>
      </div>
      <div class="w-full flex flex-row gap-2 content-around flex-nowrap">
        <input
          v-model.trim="searchedTitle"
          class="input input-xs"
          placeholder="输入标题以搜索"
        />
        <button
          class="btn btn-xs"
          :class="{ 'btn-primary': selectedTags.length > 0 }"
          @click="openTagSelector"
        >
          <Funnel class="w-4" />
          <span
            v-if="selectedTags.length > 0"
            class="badge badge-sm ml-1 p-1.5"
            >{{ selectedTags.length }}</span
          >
        </button>
      </div>
    </div>
  </div>
  <SelectedTagDialog ref="tagsSelectorDialogRef" />
</template>
