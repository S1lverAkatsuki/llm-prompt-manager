<script setup lang="ts">
import { computed, inject, type Ref } from "vue";
import { marked } from "marked";
import DOMPurify from "dompurify";
import type { Prompt } from "@/types";
import { GripVertical, Copy, Check, Pen } from "lucide-vue-next";

const props = defineProps<{
  item: Prompt;
  isExpanded: boolean;
  isCopied: boolean;
}>();

const emit = defineEmits<{
  toggleExpand: [id: string];
  copy: [id: string, content: string];
  edit: [item: Prompt];
}>();

const justDragged = inject<Ref<boolean>>("justDragged")!;
const draggingItemId = inject<Ref<string | null>>("draggingItemId")!;

const handleToggleExpand = () => {
  if (justDragged.value) return;
  emit("toggleExpand", props.item.id);
};

const isBeingDragged = computed(() => draggingItemId.value === props.item.id);

const renderedContent = computed(() =>
  DOMPurify.sanitize(marked(props.item.content) as string)
);
</script>

<template>
  <div
    class="collapse collapse-arrow bg-base-100 rounded-md border w-full"
    :class="[
      isExpanded ? 'border-primary collapse-open' : 'border-base-300',
      { 'opacity-40': isBeingDragged },
    ]"
    @click="handleToggleExpand"
  >
    <div class="collapse-title flex items-center gap-2 p-4 min-h-0 pr-12">
      <div
        class="drag-handle relative z-2 cursor-grab active:cursor-grabbing text-base-content/40 hover:text-base-content/70 shrink-0"
      >
        <GripVertical class="w-5" />
      </div>
      <div class="flex flex-col flex-1 min-w-0">
        <div class="flex flex-row gap-2 items-center mb-1">
          <p
            class="font-semibold text-base-content"
            :class="!isExpanded ? 'truncate' : 'whitespace-normal break-all'"
          >
            {{ item.title }}
          </p>
          <div class="flex gap-1">
            <span
              v-for="tag in item.tags"
              :key="tag"
              class="badge badge-sm badge-outline"
            >
              {{ tag }}
            </span>
          </div>
        </div>
        <p
          class="text-sm text-base-content/70"
          :title="item.tip"
          :class="!isExpanded ? 'truncate' : 'whitespace-normal break-all'"
        >
          {{ item.tip }}
        </p>
      </div>
    </div>
    <div
      class="collapse-content border-t border-base-300 bg-base-100/50"
      @click.stop
    >
      <div class="p-4">
        <div
          class="prose prose-sm max-w-none prose-md-preview"
          v-html="renderedContent"
        />
        <div class="mt-4 flex gap-2 justify-end">
          <button
            class="btn btn-sm btn-outline"
            @click.stop="$emit('edit', item)"
          >
            <Pen class="w-4"/>
            编辑
          </button>
          <button
            class="btn btn-sm"
            :class="isCopied ? 'btn-success' : 'btn-primary'"
            @click.stop="$emit('copy', item.id, item.content)"
          >
            <Copy v-if="!isCopied" class="w-4"/>
            <Check v-else />
            {{ isCopied ? "已复制" : "复制" }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
