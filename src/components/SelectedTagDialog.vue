<script setup lang="ts">
import { inject, ref } from "vue";
import { X } from "lucide-vue-next";
import { tagsKey } from "@/injection-keys";
import { useFilterStore } from "@/composables/useFilterStore";

const dialogRef = ref<HTMLDialogElement | null>(null);

const { tags } = inject(tagsKey)!;
const { selectedTags } = useFilterStore();

const show = () => dialogRef.value?.showModal();

defineExpose({ show });
</script>

<template>
  <dialog ref="dialogRef" id="setting" class="modal transition-none!">
    <div class="modal-box w-[50vw] max-w-3xl h-[90vh] p-0 flex flex-col">
      <div
        class="flex items-center justify-between border-b border-base-300 px-6 py-4"
      >
        <span class="flex flex-row gap-2 items-center">
          <h3 class="text-lg font-semibold text-base-content">
            选择包含的标签
          </h3>
          <button
            v-if="selectedTags.length !== 0"
            class="btn btn-xs btn-outline btn-error"
            @click="selectedTags = []"
          >
            重置选择
          </button>
        </span>

        <form method="dialog">
          <button class="btn btn-sm btn-ghost p-1">
            <X class="w-5" />
          </button>
        </form>
      </div>
      <div class="w-full flex-1 overflow-y-auto p-6 py-4 space-y-6">
        <div
          v-if="tags.length === 0"
          class="h-fit flex items-center justify-center text-base-content/50"
        >
          当前没有任何标签
        </div>
        <form class="flex flex-row gap-1 flex-wrap">
          <template v-for="tag in tags">
            <input
              class="btn btn-sm h-6"
              type="checkbox"
              :aria-label="tag"
              v-model="selectedTags"
              :value="tag"
            />
          </template>
        </form>
      </div>
    </div>
  </dialog>
</template>
