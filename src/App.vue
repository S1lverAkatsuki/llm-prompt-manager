<script setup lang="ts">
import { ref, provide } from "vue";
import { useTheme } from "@/composables/useTheme";
import { usePromptList } from "@/composables/usePromptList";
import { itemsKey } from "@/injection-keys";
import AppHeader from "@/components/AppHeader.vue";
import PromptList from "@/components/PromptList.vue";
import PromptEditor from "@/components/PromptEditor.vue";
import SettingDialog from "@/components/SettingDialog.vue";
import type { Prompt } from "@/types";

const { isInDarkMode } = useTheme();
const { items, expandedId, toggleExpand, copiedId, handleCopy } =
  usePromptList();

provide(itemsKey, items);

const promptEditorRef = ref<InstanceType<typeof PromptEditor> | null>(null);
const settingDialogRef = ref<InstanceType<typeof SettingDialog> | null>(null);

const toggleDark = () => {
  if (isInDarkMode.value !== null) {
    isInDarkMode.value = !isInDarkMode.value;
  }
};

const handleCreate = () => promptEditorRef.value?.open();

const handleEdit = (item: Prompt) => promptEditorRef.value?.open(item);

const openSetting = () => settingDialogRef.value?.show();
</script>
z
<template>
  <main class="h-screen w-screen flex flex-col items-center justify-center">
    <AppHeader
      :is-in-dark-mode="isInDarkMode"
      @toggle-dark="toggleDark"
      @open-setting="openSetting"
      @create="handleCreate"
    />
    <PromptList
      :items="items"
      :expanded-id="expandedId"
      :copied-id="copiedId"
      @toggle-expand="toggleExpand"
      @copy="handleCopy"
      @edit="handleEdit"
    />
    <PromptEditor ref="promptEditorRef" />
    <SettingDialog ref="settingDialogRef" />
  </main>
</template>
