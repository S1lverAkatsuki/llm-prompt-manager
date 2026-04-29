<script setup lang="ts">
import type { Prompt } from "@/types";
import PromptItem from "./PromptItem.vue";

defineProps<{
  items: Prompt[] | null;
  expandedId: string | null;
  copiedId: string | null;
}>();

defineEmits<{
  toggleExpand: [id: string];
  copy: [id: string, content: string];
  edit: [item: Prompt];
}>();
</script>

<template>
  <div class="bg-base-200 w-full flex-1 overflow-y-auto p-5 [scrollbar-gutter:stable_both-edges]">
    <div v-if="items == null" class="h-full flex items-center justify-center text-base-content/70">
      正在加载
    </div>
    <div v-else-if="items.length === 0" class="h-full flex items-center justify-center text-base-content/50">
      当前没有存储 Prompt
    </div>
    <div v-else class="flex flex-col gap-4">
      <PromptItem v-for="item in items" :key="item.id" :item="item" :is-expanded="expandedId === item.id"
        :is-copied="copiedId === item.id" @toggle-expand="(id) => $emit('toggleExpand', id)"
        @copy="(id, content) => $emit('copy', id, content)" @edit="(item) => $emit('edit', item)" />
    </div>
  </div>
</template>
