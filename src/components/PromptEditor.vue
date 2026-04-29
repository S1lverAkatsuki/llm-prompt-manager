<script setup lang="ts">
import { ref, inject, nextTick } from "vue";
import type { Prompt } from "@/types";
import { itemsKey } from "@/injection-keys";
import { useEditor } from "@/composables/useEditor";

const items = inject(itemsKey)!;
const dialogRef = ref<HTMLDialogElement | null>(null);

const {
  tagInput,
  editingPrompt,
  isEditorInCreateMode,
  selectedTags,
  isEmptyTag,
  canAddTag,
  handleAddTag,
  handleRemoveTag,
  openEditor,
  resetEditor,
  canSave,
  handleSave,
  handleDelete,
  deletedTimeout,
  editorContextInputRef,
  editAreaRef,
  handleInputExpanded,
  isEmptyTitle,
  isEmptyContent,
} = useEditor(items);

const open = async (item?: Prompt) => {
  openEditor(item);
  await nextTick();
  dialogRef.value?.showModal();
  handleInputExpanded();
};

const onSave = async () => {
  const shouldClose = await handleSave();
  if (shouldClose) {
    dialogRef.value?.close();
  }
};

const onDelete = async () => {
  const shouldClose = await handleDelete();
  if (shouldClose) {
    dialogRef.value?.close();
  }
};

const onDialogClose = () => {
  resetEditor();
};

defineExpose({ open });
</script>

<template>
  <dialog
    v-if="editingPrompt"
    ref="dialogRef"
    id="editor"
    class="modal"
    @close="onDialogClose"
  >
    <div class="modal-box w-[80vw] max-w-3xl h-[90vh] p-0 flex flex-col">
      <div
        class="flex items-center justify-between border-b border-base-300 px-6 py-4"
      >
        <div>
          <h3 class="text-lg font-semibold text-base-content">
            {{ isEditorInCreateMode ? "添加新的 Prompt" : "编辑 Prompt" }}
          </h3>
          <p
            v-show="!isEditorInCreateMode"
            class="text-sm text-base-content/70 mt-1"
          >
            在此修改当前 Prompt 信息
          </p>
        </div>
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
      <div
        ref="editAreaRef"
        class="flex-1 overflow-y-auto p-5 space-y-6 [scrollbar-gutter:stable_both-edges]"
      >
        <div>
          <p class="text-sm font-medium">标题 *</p>
          <input
            class="input input-sm w-full mt-2"
            type="text"
            v-model.trim="editingPrompt.title"
            placeholder="Prompt 项的标题"
            required
            :class="{ 'input-error': isEmptyTitle }"
          />
          <p
            v-show="isEmptyTitle"
            class="text-xs text-error mt-1"
          >
            请输入一个标题
          </p>
        </div>
        <div>
          <p class="text-sm font-medium">简述</p>
          <input
            type="text"
            class="input input-sm w-full mt-2"
            v-model.trim="editingPrompt.tip"
            placeholder="一句话概括 Prompt 的作用"
          />
        </div>
        <div>
          <p class="text-sm font-medium">标签</p>
          <div class="flex flex-col gap-2">
            <div class="flex flex-wrap gap-2 mt-2">
              <p
                v-if="selectedTags.length === 0"
                class="textarea-sm text-base-content/50"
              >
                当前无标签
              </p>
              <span
                v-else
                v-for="tag in selectedTags"
                :key="tag"
                class="badge badge-outline pr-1"
              >
                {{ tag }}
                <button
                  class="btn btn-ghost btn-xs btn-circle h-5 w-5 p-0"
                  @click="handleRemoveTag(tag)"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    class="w-3 h-3"
                    fill="currentColor"
                  >
                    <path
                      d="M19,6.41,17.59,5,12,10.59,6.41,5,5,6.41,10.59,12,5,17.59,6.41,19,12,13.41,17.59,19,19,17.59,13.41,12Z"
                    />
                  </svg>
                </button>
              </span>
            </div>
            <div class="join w-full">
              <input
                class="input input-sm join-item flex-1"
                v-model.trim="tagInput"
                @keyup.enter="handleAddTag"
                placeholder="输入要添加的标签项"
                :class="{ 'input-error': isEmptyTag }"
              />
              <button
                class="btn btn-sm join-item"
                @click="handleAddTag"
                :disabled="!canAddTag"
              >
                添加
              </button>
            </div>
            <p
              v-if="isEmptyTag"
              class="text-xs text-error mt-1"
            >
              重复的标签无法输入
            </p>
          </div>
        </div>
        <div>
          <p class="text-sm font-medium">内容 *</p>
          <p
            v-show="isEmptyContent"
            class="text-xs text-error mt-1"
          >
            请输入 Prompt 内容
          </p>
          <textarea
            ref="editorContextInputRef"
            class="textarea box-border w-full mt-2 resize-none overflow-x-auto overflow-y-hidden whitespace-pre [scrollbar-gutter:stable]"
            rows="3"
            v-model="editingPrompt.content"
            @input="handleInputExpanded"
            placeholder="例如：你是一只猫娘..."
            required
            :class="{ 'input-error': isEmptyContent }"
          />
        </div>
      </div>
      <div
        class="bg-base-200/50 border-t border-base-300 p-6 flex justify-end gap-2"
      >
        <button
          v-show="!isEditorInCreateMode"
          class="btn btn-outline btn-error mr-auto"
          @click="onDelete"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            class="w-4 h-4"
            fill="currentColor"
          >
            <path
              d="M9,3V4H4V6H5V19A2,2 0 0,0 7,21H17A2,2 0 0,0 19,19V6H20V4H15V3H9M7,6H17V19H7V6M9,8V17H11V8H9M13,8V17H15V8H13Z"
            />
          </svg>
          {{ deletedTimeout ? "确认删除？" : "删除" }}
        </button>
        <form method="dialog">
          <button class="btn btn-ghost text-base-content/50">取消</button>
        </form>
        <button
          class="btn btn-primary"
          @click="onSave"
          :disabled="!canSave"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            class="w-4 h-4"
            fill="currentColor"
          >
            <path
              d="M15,9H5V5H15M12,19A3,3 0 0,1 9,16A3,3 0 0,1 12,13A3,3 0 0,1 15,16A3,3 0 0,1 12,19M17,3H5C3.89,3 3,3.9 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V7L17,3Z"
            />
          </svg>
          保存
        </button>
      </div>
    </div>
  </dialog>
</template>
