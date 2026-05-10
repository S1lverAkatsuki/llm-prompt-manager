<script setup lang="ts">
import {
  ref,
  inject,
  nextTick,
  useTemplateRef,
  computed,
  watch,
} from "vue";
import type { Prompt } from "@/types";
import { promptsKey, tagsKey } from "@/injection-keys";
import { useEditor } from "@/composables/useEditor";
import { Save, Trash, X } from "lucide-vue-next";
import { useSuggestTag } from "@/composables/useSuggestTag";

const prompts = inject(promptsKey)!;
const { tags, refresh } = inject(tagsKey)!;
const dialogRef = ref<HTMLDialogElement | null>(null);

const editorContextInputRef =
  useTemplateRef<HTMLInputElement>("editorContextInput");
const editAreaRef = useTemplateRef<HTMLDivElement>("editArea");

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
  handleInputExpanded,
  isEmptyTitle,
  isEmptyContent,
} = useEditor(prompts, editorContextInputRef, editAreaRef);

const open = async (item?: Prompt) => {
  openEditor(item);
  await nextTick();
  dialogRef.value?.showModal();
  handleInputExpanded();
};

const onSave = async () => {
  const shouldClose = await handleSave();
  if (shouldClose) {
    refresh();
    dialogRef.value?.close();
  }
};

const onDelete = async () => {
  const shouldClose = await handleDelete();
  if (shouldClose) {
    refresh();
    dialogRef.value?.close();
  }
};

const onDialogClose = () => {
  resetEditor();
};

defineExpose({ open });

const { suggestTags } = useSuggestTag(tags, tagInput);

const isDropdownOpen = ref<boolean>(false);
const dropdownClass = computed<string>(() =>
  isDropdownOpen.value ? "dropdown-open" : "dropdown-close"
);

watch(
  suggestTags,
  () => {
    isDropdownOpen.value = suggestTags.value.length !== 0;
  },
  { immediate: true }
);

const submitSelectedTag = (tag: string) => {
  tagInput.value = tag;
  handleAddTag();
  isDropdownOpen.value = false;
};
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
          <button class="btn btn-sm btn-ghost p-1">
            <X class="w-5" />
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
            maxlength="50"
            :class="{ 'input-error': isEmptyTitle }"
          />
          <p v-show="isEmptyTitle" class="text-xs text-error mt-1">
            请输入一个标题
          </p>
          <p class="text-xs text-base-content/30 mt-1">
            {{ (editingPrompt?.title?.length ?? 0) }} / 50
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
                  class="btn btn-ghost btn-xs h-5 w-5 p-0"
                  @click="handleRemoveTag(tag)"
                >
                  <X class="h-3" />
                </button>
              </span>
            </div>
            <div class="join w-full">
              <div class="dropdown w-full" :class="dropdownClass">
                <input
                  class="input input-sm join-item flex-1 w-full"
                  v-model.trim="tagInput"
                  @keyup.enter="handleAddTag"
                  placeholder="输入要添加的标签项"
                  maxlength="20"
                  :class="{ 'input-error': isEmptyTag }"
                />
                <ul
                  tabindex="-1"
                  class="dropdown-content menu bg-base-100 rounded-box z-1 w-full p-2 shadow-sm"
                >
                  <template v-for="tag in suggestTags">
                    <li>
                      <span @click.prevent="submitSelectedTag(tag)">{{
                        tag
                      }}</span>
                    </li>
                  </template>
                </ul>
              </div>

              <button
                class="btn btn-sm join-item"
                @click="handleAddTag"
                :disabled="!canAddTag"
              >
                添加
              </button>
            </div>
            <p v-if="isEmptyTag" class="text-xs text-error mt-1">
              重复的标签无法输入
            </p>
            <p class="text-xs text-base-content/30 mt-1">
              {{ tagInput.length }} / 20
            </p>
          </div>
        </div>
        <div>
          <p class="text-sm font-medium">内容 *</p>
          <p v-show="isEmptyContent" class="text-xs text-error mt-1">
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
          <Trash class="w-5" />
          {{ deletedTimeout ? "确认删除？" : "删除" }}
        </button>
        <form method="dialog">
          <button class="btn btn-ghost text-base-content/50">取消</button>
        </form>
        <button class="btn btn-primary" @click="onSave" :disabled="!canSave">
          <Save class="w-5" />
          保存
        </button>
      </div>
    </div>
  </dialog>
</template>
