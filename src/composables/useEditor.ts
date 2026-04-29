import { ref, computed, nextTick, onBeforeUnmount } from "vue";
import type { Ref } from "vue";
import { Prompt, PromptData } from "@/types";
import { invoke } from "@tauri-apps/api/core";

export function useEditor(items: Ref<Prompt[] | null>) {
  const deletedTimeout = ref<NodeJS.Timeout | null>(null);

  const tagInput = ref<string>("");
  const editingPrompt = ref<PromptData | null>(null);
  const editingId = ref<string | null>(null);
  const isEditorInCreateMode = computed(() => editingId.value === null);

  const selectedTags = computed(() => editingPrompt.value?.tags ?? []);

  const isEmptyTag = computed(() => {
    const value = tagInput.value;
    return value.length > 0 && selectedTags.value.includes(value);
  });

  const canAddTag = computed(() => {
    const value = tagInput.value;
    return value.length > 0 && !isEmptyTag.value;
  });

  const handleAddTag = () => {
    if (!editingPrompt.value || !canAddTag.value) return;
    editingPrompt.value.tags.push(tagInput.value);
    tagInput.value = "";
  };

  const handleRemoveTag = (tag: string) => {
    if (!editingPrompt.value) return;
    editingPrompt.value.tags = editingPrompt.value.tags.filter(t => t !== tag);
  };

  const openEditor = (item?: Prompt) => {
    if (item) {
      editingPrompt.value = {
        title: item.title,
        tip: item.tip,
        content: item.content,
        tags: [...item.tags],
      };
      editingId.value = item.id;
    } else {
      editingPrompt.value = { title: "", tip: "", content: "", tags: [] };
      editingId.value = null;
    }
    tagInput.value = "";
    deletedTimeout.value = null;
  };

  const resetEditor = () => {
    tagInput.value = "";
    editingPrompt.value = null;
    editingId.value = null;
    deletedTimeout.value = null;
  };

  const isEmptyTitle = computed<boolean>(
    () => editingPrompt.value?.title.length === 0
  );
  const isEmptyContent = computed<boolean>(
    () => editingPrompt.value?.content.length === 0
  );
  const canSave = computed<boolean>(
    () => !isEmptyContent.value && !isEmptyTitle.value
  );

  const handleSave = async (): Promise<boolean> => {
    const data = editingPrompt.value;
    if (!data) return false;

    if (isEditorInCreateMode.value) {
      try {
        const createdPrompt = await invoke<Prompt>("create", { draft: data });
        if (!items.value) {
          items.value = [createdPrompt];
        } else {
          items.value.push(createdPrompt);
        }
      } catch (e) {
        console.error(e);
      }
      return true;
    }

    if (!items.value || !editingId.value) return false;

    const index = items.value.findIndex(item => item.id === editingId.value);

    if (index !== -1) {
      const updatedPrompt: Prompt = {
        ...items.value[index],
        ...data,
        id: editingId.value,
      };
      try {
        await invoke("update", {
          newPrompt: updatedPrompt,
        });
      } catch (e) {
        console.error(e);
      }
      items.value[index] = updatedPrompt;
    }

    return true;
  };

  const handleDelete = async (): Promise<boolean> => {
    if (!editingPrompt.value || !items.value || !editingId.value) return false;

    if (deletedTimeout.value) {
      try {
        await invoke("delete", { deletedId: editingId.value });
      } catch (e) {
        console.error(e);
      }

      items.value = items.value.filter(item => item.id !== editingId.value);

      clearTimeout(deletedTimeout.value);
      deletedTimeout.value = null;

      return true;
    }

    const RESET_MILLISECOND = 2000;
    deletedTimeout.value = setTimeout(() => {
      deletedTimeout.value = null;
    }, RESET_MILLISECOND);

    return false;
  };

  onBeforeUnmount(() => {
    if (deletedTimeout.value) {
      clearTimeout(deletedTimeout.value);
    }
  });

  const editorContextInputRef = ref<HTMLInputElement | null>(null);
  const editAreaRef = ref<HTMLElement | null>(null);

  const handleInputExpanded = async () => {
    await nextTick();

    if (!editorContextInputRef.value || !editAreaRef.value) return;

    const parentScroll = editAreaRef.value.scrollTop;

    editorContextInputRef.value.style.height = "auto";
    editorContextInputRef.value.style.height = `${editorContextInputRef.value.scrollHeight + 8}px`;

    editAreaRef.value.scrollTop = parentScroll;
  };

  return {
    tagInput,
    editingPrompt,
    editingId,
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
  };
}
