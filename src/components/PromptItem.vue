<script setup lang="ts">
import type { Prompt } from "@/types";

defineProps<{
  item: Prompt;
  isExpanded: boolean;
  isCopied: boolean;
}>();

defineEmits<{
  toggleExpand: [id: string];
  copy: [id: string, content: string];
  edit: [item: Prompt];
}>();
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 rounded-md border w-full"
    :class="isExpanded ? 'border-primary collapse-open' : 'border-base-300'">
    <input type="checkbox" :checked="isExpanded" @change="$emit('toggleExpand', item.id)" />
    <div class="collapse-title flex items-center gap-2 p-4 min-h-0 pr-12">
      <div class="flex flex-col flex-1 min-w-0">
        <div class="flex flex-row gap-2 items-center mb-1">
          <p class="font-semibold text-base-content"
            :class="!isExpanded ? 'truncate' : 'whitespace-normal break-all'">{{ item.title }}
          </p>
          <div class="flex gap-1">
            <span v-for="tag in item.tags" :key="tag" class="badge badge-sm badge-outline">
              {{ tag }}
            </span>
          </div>
        </div>
        <p class="text-sm text-base-content/70" :title="item.tip"
          :class="!isExpanded ? 'truncate' : 'whitespace-normal break-all'">
          {{ item.tip }}
        </p>
      </div>
    </div>
    <div class="collapse-content border-t border-base-300 bg-base-100/50" @click.stop>
      <div class="p-4">
        <div class="prose prose-sm max-w-none">
          <p class="text-base-content whitespace-pre-wrap break-all">{{ item.content }}</p>
        </div>
        <div class="mt-4 flex gap-2 justify-end">
          <button class="btn btn-sm btn-outline" @click.stop="$emit('edit', item)">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4">
              <path d="M20.71,7.04C21.1,6.65 21.1,6 20.71,5.63L18.37,3.29C18,2.9 17.35,2.9 16.96,3.29L15.12,5.12L18.87,8.87M3,17.25V21H6.75L17.81,9.93L14.06,6.18L3,17.25Z" />
            </svg>
            编辑
          </button>
          <button class="btn btn-sm" :class="isCopied ? 'btn-success' : 'btn-primary'"
            @click.stop="$emit('copy', item.id, item.content)">
            <svg v-if="!isCopied" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
              fill="currentColor" class="w-4 h-4">
              <path d="M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z" />
            </svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
              class="w-4 h-4">
              <path d="M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z" />
            </svg>
            {{ isCopied ? "已复制" : "复制" }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
