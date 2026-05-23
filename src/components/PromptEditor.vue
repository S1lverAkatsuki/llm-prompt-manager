<script setup lang="ts">
import { ref, inject, nextTick, useTemplateRef, computed, watch } from "vue";
import type { Prompt } from "@/types";
import { promptsKey, tagsKey } from "@/injection-keys";
import { useEditor } from "@/composables/useEditor";
import { useAiConfig } from "@/composables/useAiConfig";
import { useErrorLog } from "@/composables/useErrorLog";
import { useToast } from "@/composables/useToast";
import { Save, Trash, X } from "lucide-vue-next";
import { useSuggestTag } from "@/composables/useSuggestTag";
import { MAX_TAG_LENGTH, MAX_TITLE_LENGTH } from "@/constants.ts";
import { invoke } from "@tauri-apps/api/core";

const prompts = inject(promptsKey)!;
const { tags, refresh } = inject(tagsKey)!;
const { isAiEnabled, loadAiConfig } = useAiConfig();
const { pushErrorLog } = useErrorLog();
const { pushToast } = useToast();
const dialogRef = ref<HTMLDialogElement | null>(null);

const editorContextInputRef =
  useTemplateRef<HTMLInputElement>("editorContextInput");
const editAreaRef = useTemplateRef<HTMLDivElement>("editArea");

const {
  editingId,
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
  isUserPromptEmpty.value = false;
  if (!item) {
    await loadAiConfig();
    editorCreateStep.value = isAiEnabled.value
      ? EditorCreateStep.LlmCreate
      : EditorCreateStep.HandWriting;
  }
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
  isUserPromptEmpty.value = false;
  userPrompt.value = "";
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

enum EditorCreateStep {
  LlmCreate,
  HandWriting,
}

const editorCreateStep = ref<EditorCreateStep>(EditorCreateStep.LlmCreate);

const dialogTitle = computed<string>(() => {
  return isEditorInCreateMode.value ? "添加新的 Prompt" : "编辑 Prompt";
});

const dialogSubTitle = computed<string>(() => {
  if (!isEditorInCreateMode.value) return "在此修改当前 Prompt 信息";
  switch (editorCreateStep.value) {
    case EditorCreateStep.LlmCreate:
      return "由给定的模型自动生成 Prompt";
    case EditorCreateStep.HandWriting:
      return "填写将要存储的 Prompt";
  }
});

const userPrompt = ref<string>("");
const isUserPromptEmpty = ref<boolean>(false);

const isGenerating = ref<boolean>(false);

const cancelGeneratingTimeout = ref<ReturnType<typeof setTimeout> | null>(null);

const handleCancelGenerating = async () => {
  if (!isGenerating.value) {
    editorCreateStep.value = EditorCreateStep.HandWriting;
    nextTick().then(() => handleInputExpanded());
    return;
  }
  if (cancelGeneratingTimeout.value) {
    clearTimeout(cancelGeneratingTimeout.value);
    cancelGeneratingTimeout.value = null;
    isGenerating.value = false;
    editorCreateStep.value = EditorCreateStep.HandWriting;
    await nextTick();
    handleInputExpanded();
  } else {
    cancelGeneratingTimeout.value = setTimeout(() => {
      cancelGeneratingTimeout.value = null;
    }, 2000);
  }
};

const handleGeneratePrompt = async () => {
  if (!userPrompt.value.trim()) {
    isUserPromptEmpty.value = true;
    return;
  }

  isUserPromptEmpty.value = false;
  isGenerating.value = true;
  try {
    const generateResult = await invoke<Prompt>("ai_generate", {
      userPrompt: userPrompt.value,
    });
    editingPrompt.value = generateResult!;
    editorCreateStep.value = EditorCreateStep.HandWriting;
    await nextTick();
    handleInputExpanded();
  } catch (e) {
    pushErrorLog("AI 生成失败", e);
    pushToast("AI 生成失败，请检查配置或查看错误日志", "error");
  } finally {
    isGenerating.value = false;
  }
};

watch(userPrompt, value => {
  if (value.trim()) {
    isUserPromptEmpty.value = false;
  }
});

const onCopyToNewItem = () => {
  // 这里只需要改一下 ID 为空就能变成创建新条目的模式
  // 后端会自己生成 ID 的
  // 真是太巧合了
  const currentEdithingId = editingId.value;
  const currentEdithingMeta = editingPrompt.value;
  editingId.value = null;
  editingPrompt.value!.title = currentEdithingMeta?.title + " (副本)";
  handleSave();
  pushToast(`已复制到 ${editingPrompt.value!.title}`, "success");
  editingId.value = currentEdithingId;
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
    <div class="modal-box w-[80vw] max-w-3xl p-0 flex flex-col h-[90vh]">
      <div
        class="flex items-center justify-between border-b border-base-300 px-6 py-4"
      >
        <div>
          <h3 class="text-lg font-semibold text-base-content">
            {{ dialogTitle }}
          </h3>
          <p
            v-if="dialogSubTitle.length > 0"
            class="text-sm text-base-content/70 mt-1"
          >
            {{ dialogSubTitle }}
          </p>
        </div>
        <form method="dialog">
          <button class="btn btn-sm btn-ghost p-1">
            <X class="w-5" />
          </button>
        </form>
      </div>
      <template
        v-if="
          isEditorInCreateMode &&
          editorCreateStep === EditorCreateStep.LlmCreate
        "
      >
        <div
          class="flex-1 overflow-y-auto p-5 flex flex-col gap-3 [scrollbar-gutter:stable_both-edges]"
        >
          <div class="text-sm text-base-content/70">填写一些描述性文本</div>
          <textarea
            class="textarea w-full flex mx-auto resize-none flex-1"
            :class="{ 'textarea-error': isUserPromptEmpty }"
            placeholder="说说希望得到什么样的提示词"
            v-model="userPrompt"
            :disabled="isGenerating"
          ></textarea>
          <p v-if="isUserPromptEmpty" class="text-xs text-error">
            请输入用于生成提示词的需求描述
          </p>
          <p class="text-xs text-base-content/40">可前往设置中修改模型配置</p>
        </div>
        <div
          class="bg-base-200/50 border-t border-base-300 p-6 flex justify-end gap-2"
        >
          <button class="btn" @click="handleCancelGenerating">
            {{ cancelGeneratingTimeout ? "确认放弃所有更改？" : "手动编写" }}
          </button>
          <button
            class="btn btn-primary"
            :disabled="isGenerating"
            @click="handleGeneratePrompt"
          >
            {{ isGenerating ? "生成中..." : "生成" }}
          </button>
        </div>
      </template>
      <template
        v-if="
          !isEditorInCreateMode ||
          (isEditorInCreateMode &&
            editorCreateStep === EditorCreateStep.HandWriting)
        "
      >
        <div
          ref="editArea"
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
              :maxlength="MAX_TITLE_LENGTH"
              :class="{ 'input-error': isEmptyTitle }"
            />
            <p v-show="isEmptyTitle" class="text-xs text-error mt-1">
              请输入一个标题
            </p>
            <p class="text-xs text-base-content/30 mt-1">
              {{ editingPrompt?.title?.length ?? 0 }} / {{ MAX_TITLE_LENGTH }}
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
                    :maxlength="MAX_TAG_LENGTH"
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
                重复的标签无法添加
              </p>
              <p class="text-xs text-base-content/30 mt-1">
                {{ tagInput.length }} / {{ MAX_TAG_LENGTH }}
              </p>
            </div>
          </div>
          <div>
            <span class="text-sm font-medium">内容 *</span>
            <span class="text-sm font-medium text-base-content/50"
              >（可使用 CommonMarkdown 语法）</span
            >
            <p v-show="isEmptyContent" class="text-xs text-error mt-1">
              请输入 Prompt 内容
            </p>
            <textarea
              ref="editorContextInput"
              class="textarea box-border w-full mt-2 resize-y overflow-x-auto overflow-y-hidden whitespace-pre [scrollbar-gutter:stable]"
              rows="6"
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
          <button
            class="btn btn-ghost text-base-content"
            @click="onCopyToNewItem"
          >
            复制到新项
          </button>
          <button class="btn btn-primary" @click="onSave" :disabled="!canSave">
            <Save class="w-5" />
            保存
          </button>
        </div>
      </template>
    </div>
  </dialog>
</template>
