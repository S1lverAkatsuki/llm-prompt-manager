<script setup lang="ts">
import { inject, ref, provide } from "vue";
import type { Prompt } from "@/types";
import PromptItem from "@/components/PromptItem.vue";
import draggable from "vuedraggable";
import { itemsKey } from "@/injection-keys";

const items = inject(itemsKey)!;

const props = defineProps<{
  expandedId: string | null;
  copiedId: string | null;
}>();

const emit = defineEmits<{
  toggleExpand: [id: string];
  copy: [id: string, content: string];
  edit: [item: Prompt];
  reorder: [];
}>();

const isDragging = ref(false);
const draggingItemId = ref<string | null>(null);
const collapsedIdForDrag = ref<string | null>(null);

const justDragged = ref(false);

provide("justDragged", justDragged);
provide("draggingItemId", draggingItemId);

const handleDragStart = (evt: { oldIndex: number }) => {
  isDragging.value = true;

  if (!items.value) return;
  const draggedItem = items.value[evt.oldIndex];
  if (!draggedItem) return;

  draggingItemId.value = draggedItem.id;

  if (draggedItem.id === props.expandedId) {
    collapsedIdForDrag.value = draggedItem.id;
    emit("toggleExpand", draggedItem.id);
  }
};

const handleDragEnd = () => {
  isDragging.value = false;
  draggingItemId.value = null;

  if (collapsedIdForDrag.value) {
    emit("toggleExpand", collapsedIdForDrag.value);
    collapsedIdForDrag.value = null;
  }

  justDragged.value = true;
  emit("reorder");
  setTimeout(() => {
    justDragged.value = false;
  }, 0);
};
</script>

<template>
  <div
    class="bg-base-200 w-full flex-1 overflow-y-auto p-5 pl-0 pr-2 [scrollbar-gutter:stable_both-edges]"
    :class="{ 'is-dragging': isDragging }"
  >
    <div
      v-if="items == null"
      class="h-full flex items-center justify-center text-base-content/70"
    >
      正在加载
    </div>
    <div
      v-else-if="items.length === 0"
      class="h-full flex items-center justify-center text-base-content/50"
    >
      当前没有存储 Prompt
    </div>
    <draggable
      v-else
      :list="items"
      item-key="id"
      handle=".drag-handle"
      :animation="200"
      ghost-class="sortable-ghost"
      :force-fallback="true"
      fallback-class="sortable-fallback"
      fallback-on-body="true"
      class="flex flex-col gap-3"
      @start="handleDragStart"
      @end="handleDragEnd"
    >
      <template #item="{ element }">
        <PromptItem
          :item="element"
          :is-expanded="expandedId === element.id"
          :is-copied="copiedId === element.id"
          @toggle-expand="id => $emit('toggleExpand', id)"
          @copy="(id, content) => $emit('copy', id, content)"
          @edit="item => $emit('edit', item)"
        />
      </template>
    </draggable>
  </div>
</template>

<style>
.sortable-fallback {
  opacity: 1 !important;
  background-color: var(--color-base-100);
  border-radius: var(--rounded-box, 0.5rem);
  box-shadow:
    0 4px 6px -1px rgb(0 0 0 / 0.1),
    0 10px 15px -3px rgb(0 0 0 / 0.1),
    0 0 0 1px var(--color-primary);
  z-index: 9999;
  pointer-events: none !important;
  height: auto !important;
  min-height: auto !important;
  grid-template-rows: auto 0fr !important;
}

.sortable-fallback .collapse-content {
  display: none;
}

.sortable-fallback .collapse-title:after {
  transform: translateY(-100%) rotate(45deg) !important;
}

.sortable-ghost {
  opacity: 0.3;
  user-select: none;
}

.is-dragging * {
  user-select: none !important;
}
</style>
