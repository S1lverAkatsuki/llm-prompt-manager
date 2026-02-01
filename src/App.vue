<script setup lang="ts">
import { ref, computed, nextTick, onBeforeUnmount, onMounted } from "vue";
import { Prompt } from "@/types";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { invoke } from "@tauri-apps/api/core";
import { revealItemInDir } from "@tauri-apps/plugin-opener";

const copiedId = ref<string | null>(null);
const copyTimeout = ref<NodeJS.Timeout | null>(null);

const deletedTimeout = ref<NodeJS.Timeout | null>(null);

const editorRef = ref<HTMLDialogElement | null>(null);
const settingRef = ref<HTMLDialogElement | null>(null);

const data_path = ref<string>("");
const version = ref<string>("");

const expandedId = ref<string | null>(null);

const items = ref<Prompt[] | null>(null);

onMounted(() => {
  invoke<Prompt[]>("read")
    .then(data => items.value = data)
    .catch(e => {
      items.value = [];
      console.error(e);
    });
  invoke<string>("get_data_path")
    .then(path => data_path.value = path)
    .catch(e => console.error(e));
  invoke<string>("get_version")
    .then(x => version.value = x)
    .catch(e => console.error(e));
});

const toggleExpand = (id: string) => {
  expandedId.value = expandedId.value === id ? null : id;
};

const handleCopy = async (id: string, content: string) => {
  try {
    await writeText(content);
    copiedId.value = id;

    // 小心连点多个按钮
    if (copyTimeout.value) {
      clearTimeout(copyTimeout.value);
    }

    const RESET_MILLISECOND = 2000;
    copyTimeout.value = setTimeout(() => {
      copiedId.value = null;
    }, RESET_MILLISECOND);
  } catch (err) {
    // 吞掉
  }
};

const handleDelete = async () => {
  if (!editingPrompt.value || !items.value) return;


  // 有计时器的时候肯定进确认阶段了
  if (deletedTimeout.value) {
    try {
      await invoke("delete", { deletedId: editingPrompt.value?.id });
    } catch (e) {
      console.error(e);
    }

    items.value = items.value.filter(item => item.id !== editingPrompt.value?.id);

    clearTimeout(deletedTimeout.value);
    deletedTimeout.value = null;

    editorRef.value?.close();
    resetEditor();
    return;
  }

  const RESET_MILLISECOND = 2000;
  deletedTimeout.value = setTimeout(() => {
    deletedTimeout.value = null;
  }, RESET_MILLISECOND);

};

onBeforeUnmount(() => {
  if (copyTimeout.value) {
    clearTimeout(copyTimeout.value);
  }
  if (deletedTimeout.value) {
    clearTimeout(deletedTimeout.value);
  }
})

const tagInput = ref<string>("");

const editingPrompt = ref<Prompt | null>(null);

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
}

const handleRemoveTag = (tag: string) => {
  if (!editingPrompt.value) return;
  editingPrompt.value.tags = editingPrompt.value.tags.filter(t => t !== tag)
}

const openSetting = () => {
  settingRef.value?.showModal();
}

const openEditor = async (item: Prompt) => {
  // tags 是引用，必须复制才能和原来的脱钩
  editingPrompt.value = {
    ...item,
    tags: [...item.tags]
  };
  tagInput.value = "";

  // 模板里写的是 v-if="editedPrompt"，不先等创建完对象再来，会让第一次点击编辑按钮无响应
  await nextTick();

  editorRef.value?.showModal();
};

const resetEditor = () => {
  tagInput.value = "";
  editingPrompt.value = null;
  deletedTimeout.value = null;
};

const canSave = computed<boolean>(() => !isEmptyContent.value && !isEmptyTitle.value);

const handleCreate = async () => {
  try {
    items.value = await invoke("create");
  } catch {
    // 吞掉
  }
}

const handleSave = async () => {
  if (!editingPrompt.value || !items.value) return;

  const index = items.value.findIndex(item => item.id === editingPrompt.value?.id);

  if (index !== -1) {
    try {
      await invoke("update", {
        newPrompt: {
          ...items.value[index],
          ...editingPrompt.value
        }
      });

    } catch {
      // 吞掉
    }
    items.value[index] = {
      ...items.value[index],
      ...editingPrompt.value
    };
  }

  editorRef.value?.close();
}

const editorContextInputRef = ref<HTMLInputElement | null>(null);
const editAreaRef = ref<HTMLElement | null>(null);


const handleInputExpanded = () => {
  if (!editorContextInputRef.value || !editAreaRef.value) return;

  // 不保存状态会在编辑的时候强制滚动当编辑位置
  const parentScroll = editAreaRef.value.scrollTop;

  editorContextInputRef.value.style.height = "auto";
  editorContextInputRef.value.style.height = `${editorContextInputRef.value.scrollHeight}px`;

  editAreaRef.value.scrollTop = parentScroll;
}

const isEmptyTitle = computed<boolean>(() => editingPrompt.value?.title.length === 0);
const isEmptyContent = computed<boolean>(() => editingPrompt.value?.content.length === 0);

</script>

<template>
  <main class="h-screen w-screen flex flex-col items-center justify-center">
    <div class="w-full h-20 bg-base-100 border-base-300 border-b">
      <div class="h-full flex flex-row items-center p-4 gap-2 flex-nowrap">
        <a href="https://github.com/S1lverAkatsuki/llm-prompt-manager" target="_blank" rel="noopener noreferrer"
          class="text-primary font-bold text-xl shrink-0 hover:brightness-80 cursor-pointer" title="前往Github查看">
          LLM-Prompt-Manager
        </a>
        <label class="swap swap-rotate btn btn-circle btn-ghost btn-sm text-base-content hover:bg-base-300">
          <input type="checkbox" class="theme-controller" value="dark" />

          <!-- sun icon -->
          <svg class="swap-off w-5 h-5 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" />
          </svg>

          <!-- moon icon -->
          <svg class="swap-on w-5 h-5 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" />
          </svg>
        </label>
        <button class="btn btn-circle btn-ghost btn-sm text-base-content hover:bg-base-300" @click="openSetting">
          <svg class="fill-current w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path
              d="M12,8A4,4 0 0,1 16,12A4,4 0 0,1 12,16A4,4 0 0,1 8,12A4,4 0 0,1 12,8M12,10A2,2 0 0,0 10,12A2,2 0 0,0 12,14A2,2 0 0,0 14,12A2,2 0 0,0 12,10M10,22C9.75,22 9.54,21.82 9.5,21.58L9.13,18.93C8.5,18.68 7.96,18.34 7.44,17.94L4.95,18.95C4.73,19.03 4.46,18.95 4.34,18.73L2.34,15.27C2.21,15.05 2.27,14.78 2.46,14.63L4.57,12.97L4.5,12L4.57,11L2.46,9.37C2.27,9.22 2.21,8.95 2.34,8.73L4.34,5.27C4.46,5.05 4.73,4.96 4.95,5.05L7.44,6.05C7.96,5.66 8.5,5.32 9.13,5.07L9.5,2.42C9.54,2.18 9.75,2 10,2H14C14.25,2 14.46,2.18 14.5,2.42L14.87,5.07C15.5,5.32 16.04,5.66 16.56,6.05L19.05,5.05C19.27,4.96 19.54,5.05 19.66,5.27L21.66,8.73C21.79,8.95 21.73,9.22 21.54,9.37L19.43,11L19.5,12L19.43,13L21.54,14.63C21.73,14.78 21.79,15.05 21.66,15.27L19.66,18.73C19.54,18.95 19.27,19.04 19.05,18.95L16.56,17.95C16.04,18.34 15.5,18.68 14.87,18.93L14.5,21.58C14.46,21.82 14.25,22 14,22H10M11.25,4L10.88,6.61C9.68,6.86 8.62,7.5 7.85,8.39L5.44,7.35L4.69,8.65L6.8,10.2C6.4,11.37 6.4,12.64 6.8,13.8L4.68,15.36L5.43,16.66L7.86,15.62C8.63,16.5 9.68,17.14 10.87,17.38L11.24,20H12.76L13.13,17.39C14.32,17.14 15.37,16.5 16.14,15.62L18.57,16.66L19.32,15.36L17.2,13.81C17.6,12.64 17.6,11.37 17.2,10.2L19.31,8.65L18.56,7.35L16.15,8.39C15.38,7.5 14.32,6.86 13.12,6.62L12.75,4H11.25Z" />
          </svg>
        </button>
        <button class="btn ml-auto btn-primary whitespace-nowrap pl-5 pr-5" @click="handleCreate">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
            class="w-5 h-5 inline-block mr-2">
            <path d="M19,13H13V19H11V13H5V11H11V5H13V11H19V13Z" />
          </svg>
          添加新项
        </button>
      </div>
    </div>
    <div class="bg-base-200 w-full flex-1 overflow-y-auto p-5 [scrollbar-gutter:stable_both-edges]">
      <div v-if="items == null" class="h-full flex items-center justify-center text-base-content/70">
        正在加载
      </div>
      <div v-else-if="items.length === 0" class="h-full flex items-center justify-center text-base-content/50">
        当前没有存储 Prompt
      </div>
      <div v-else class="flex flex-col gap-4">
        <div v-for="item in items" :key="item.id" class="collapse collapse-arrow bg-base-100 rounded-md border w-full"
          :class="expandedId === item.id ? 'border-primary collapse-open' : 'border-base-300'">
          <input type="checkbox" :checked="expandedId === item.id" @change="toggleExpand(item.id)" />
          <div class="collapse-title flex items-center gap-2 p-4 min-h-0 pr-12">
            <div class="flex flex-col flex-1 min-w-0">
              <div class="flex flex-row gap-2 items-center mb-1">
                <p class="font-semibold text-base-content"
                  :class="expandedId !== item.id ? 'truncate' : 'whitespace-normal break-all'">{{ item.title }}
                </p>
                <div class="flex gap-1">
                  <span v-for="tag in item.tags" :key="tag" class="badge badge-sm badge-outline">
                    {{ tag }}
                  </span>
                </div>
              </div>
              <p class="text-sm text-base-content/70" :title="item.tip"
                :class="expandedId !== item.id ? 'truncate' : 'whitespace-normal break-all'">
                {{ item.tip }}
              </p>
            </div>
          </div>
          <div class="collapse-content border-t border-base-300 bg-base-100/50" @click.stop>
            <div class="p-4">
              <div class="prose prose-sm max-w-none">
                <p class="text-base-content">{{ item.content }}</p>
              </div>
              <div class="mt-4 flex gap-2 justify-end">
                <button class="btn btn-sm btn-outline" @click.stop="openEditor(item)">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4">
                    <path
                      d="M20.71,7.04C21.1,6.65 21.1,6 20.71,5.63L18.37,3.29C18,2.9 17.35,2.9 16.96,3.29L15.12,5.12L18.87,8.87M3,17.25V21H6.75L17.81,9.93L14.06,6.18L3,17.25Z" />
                  </svg>
                  编辑
                </button>
                <button class="btn btn-sm" :class="copiedId === item.id ? 'btn-success' : 'btn-primary'"
                  @click.stop="handleCopy(item.id, item.content)">
                  <svg v-if="copiedId !== item.id" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
                    fill="currentColor" class="w-4 h-4">
                    <path
                      d="M19,21H8V7H19M19,5H8A2,2 0 0,0 6,7V21A2,2 0 0,0 8,23H19A2,2 0 0,0 21,21V7A2,2 0 0,0 19,5M16,1H4A2,2 0 0,0 2,3V17H4V3H16V1Z" />
                  </svg>
                  <svg v-else xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"
                    class="w-4 h-4">
                    <path d="M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z" />
                  </svg>
                  {{ copiedId === item.id ? "已复制" : "复制" }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <dialog v-if="editingPrompt" ref="editorRef" id="editor" class="modal" @close="resetEditor">
      <div class="modal-box w-[80vw] max-w-3xl h-[90vh] p-0 flex flex-col">
        <div class="flex items-center justify-between border-b border-base-300 px-6 py-4">
          <div>
            <h3 class="text-lg font-semibold text-base-content">编辑 Prompt</h3>
            <p class="text-sm text-base-content/70 mt-1">在此修改当前 Prompt 信息</p>
          </div>
          <form method="dialog">
            <button class="btn btn-sm btn-circle btn-ghost">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-4 h-4" fill="currentColor">
                <path
                  d="M19,6.41,17.59,5,12,10.59,6.41,5,5,6.41,10.59,12,5,17.59,6.41,19,12,13.41,17.59,19,19,17.59,13.41,12Z" />
              </svg>
            </button>
          </form>
        </div>
        <div ref="editAreaRef" class="flex-1 overflow-y-auto p-5 space-y-6 [scrollbar-gutter:stable_both-edges]">
          <div>
            <p class="text-sm font-medium">标题 *</p>
            <input class="input input-sm w-full mt-2" type="text" v-model.trim="editingPrompt.title"
              placeholder="Prompt 项的标题" required :class="{ 'input-error': isEmptyTitle }" />
            <p v-show="isEmptyTitle" class="text-xs text-error mt-1">
              请输入一个标题
            </p>
          </div>
          <div>
            <p class="text-sm font-medium">简述</p>
            <input type="text" class="input input-sm w-full mt-2" v-model.trim="editingPrompt.tip"
              placeholder="简短描述一下 Prompt 的用途" />
          </div>
          <div>
            <p class="text-sm font-medium">标签</p>
            <div class="flex flex-col gap-2">
              <div class="flex flex-wrap gap-2 mt-2">
                <p v-if="selectedTags.length === 0" class="textarea-sm text-base-content/50">当前无标签</p>
                <span v-else v-for="tag in selectedTags" :key="tag" class="badge badge-outline pr-1">
                  {{ tag }}
                  <button class="btn btn-ghost btn-xs btn-circle h-5 w-5 p-0" @click="handleRemoveTag(tag)">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-3 h-3" fill="currentColor">
                      <path
                        d="M19,6.41,17.59,5,12,10.59,6.41,5,5,6.41,10.59,12,5,17.59,6.41,19,12,13.41,17.59,19,19,17.59,13.41,12Z" />
                    </svg>
                  </button>
                </span>
              </div>
              <div class="join w-full">
                <input class="input input-sm join-item flex-1" v-model.trim="tagInput" @keyup.enter="handleAddTag"
                  placeholder="输入要添加的标签项" :class="{ 'input-error': isEmptyTag }" />
                <button class="btn btn-sm join-item" @click="handleAddTag" :disabled="!canAddTag">添加</button>
              </div>
              <p v-if="isEmptyTag" class="text-xs text-error mt-1">
                重复的标签无法输入
              </p>
            </div>
          </div>
          <div>
            <p class="text-sm font-medium">内容 *</p>
            <p v-show="isEmptyContent" class="text-xs text-error mt-1">
              请输入 Prompt 内容
            </p>
            <textarea ref="editorContextInputRef" class="textarea w-full mt-2 resize-none overflow-hidden" rows="3"
              v-model="editingPrompt.content" @input="handleInputExpanded" placeholder="Prompt 的内容" required
              :class="{ 'input-error': isEmptyContent }" />
          </div>
        </div>
        <div class="bg-base-200/50 border-t border-base-300 p-6 flex justify-end gap-2">
          <button class="btn btn-outline btn-error mr-auto" @click="handleDelete">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-4 h-4" fill="currentColor">
              <path
                d="M9,3V4H4V6H5V19A2,2 0 0,0 7,21H17A2,2 0 0,0 19,19V6H20V4H15V3H9M7,6H17V19H7V6M9,8V17H11V8H9M13,8V17H15V8H13Z" />
            </svg>
            {{ deletedTimeout ? "确认删除？" : "删除" }}
          </button>
          <form method="dialog">
            <button class="btn btn-ghost text-base-content/50">取消</button>
          </form>
          <button class="btn btn-primary" @click="handleSave" :disabled="!canSave">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-4 h-4" fill="currentColor">
              <path
                d="M15,9H5V5H15M12,19A3,3 0 0,1 9,16A3,3 0 0,1 12,13A3,3 0 0,1 15,16A3,3 0 0,1 12,19M17,3H5C3.89,3 3,3.9 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V7L17,3Z" />
            </svg>
            保存
          </button>
        </div>
      </div>
    </dialog>

    <dialog ref="settingRef" id="setting" class="modal transition-none!">
      <div class="modal-box w-[50vw] max-w-3xl h-[90vh] p-0 flex flex-col">
        <div class="flex items-center justify-between border-b border-base-300 px-6 py-4">
          <h3 class="text-lg font-semibold text-base-content">设置</h3>
          <form method="dialog">
            <button class="btn btn-sm btn-circle btn-ghost">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-4 h-4" fill="currentColor">
                <path
                  d="M19,6.41,17.59,5,12,10.59,6.41,5,5,6.41,10.59,12,5,17.59,6.41,19,12,13.41,17.59,19,19,17.59,13.41,12Z" />
              </svg>
            </button>
          </form>
        </div>
        <div class="flex-1 overflow-y-auto p-6 py-4 space-y-6">
          <div>
            <p class="text-sm font-medium">配置文件位置</p>
            <p class="text-sm text-base-content/50 mt-2 mb-2">跟随 Tauri 默认应用配置，很显然没有改的必要</p>
            <a class="link link-info w-full whitespace-normal break-all" title="用资源管理器打开"
              @click="revealItemInDir(data_path!)">{{ data_path }}</a>
          </div>
          <div>
            <p class="text-sm font-medium">关于</p>
            <div class="card w-full bg-base-200/50 card-sm shadow mt-2">
              <div class="card-body">
                <div class="flex flex-row gap-2 items-center">
                  <h2 class="card-title">LLM-Prompt-Manager</h2>
                  <span class="inline ml-auto text-sm text-base-content">{{ version }}</span>
                </div>
                <p class="text-sm text-base-content/50">aaa</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </dialog>
  </main>
</template>
