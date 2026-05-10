<script setup lang="ts">
import {
  computed,
  inject,
  onMounted,
  onUnmounted,
  ref,
  watch,
  type Ref,
} from "vue";
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

const isBeingDragged = computed<boolean>(
  () => draggingItemId.value === props.item.id
);

const renderedContent = computed<string>(
  () => DOMPurify.sanitize(marked(props.item.content) as string) // marked 有异步扩展后就是异步返回了，当然这里没有，所以用类型断言
);

const resizeObserver = ref<ResizeObserver | null>(null);
const tagsContainerParentRef = ref<HTMLElement | null>(null);
const tagsContainerRef = ref<HTMLElement | null>(null);
const isTagsContainerExpanded = ref<boolean>(false);

const START_TIME_MS: number = 1000;
const RESET_TIME_MS: number = 2000;

const updatedIsTagsContainerExpanded = () => {
  if (
    tagsContainerRef.value === null ||
    tagsContainerParentRef.value === null
  ) {
    return;
  }
  const child = tagsContainerRef.value;
  const parent = tagsContainerParentRef.value;
  isTagsContainerExpanded.value = child.scrollWidth > parent.clientWidth;
};

onMounted(() => {
  if (tagsContainerRef.value) {
    resizeObserver.value = new ResizeObserver(() =>
      updatedIsTagsContainerExpanded()
    );
    resizeObserver.value.observe(tagsContainerRef.value);
  }
  updatedIsTagsContainerExpanded();
  startTimeout = setTimeout(() => {
    requestAnimationFrame(scrollTagsContainer);
  }, START_TIME_MS);
});

onUnmounted(() => {
  if (resizeObserver.value) {
    resizeObserver.value.disconnect();
  }
  if (endTimeout !== null) {
    clearTimeout(endTimeout);
    endTimeout = null;
  }
  if (startTimeout !== null) {
    clearTimeout(startTimeout);
    startTimeout = null;
  }
});

let start: number | null = null;
let startTimeout: NodeJS.Timeout | null = null;
let endTimeout: NodeJS.Timeout | null = null;

const scrollTagsContainer = () => {
  if (
    props.isExpanded === true ||
    isTagsContainerExpanded.value === false ||
    tagsContainerRef.value === null
  )
    return;
  if (start === null) {
    start = Date.now();
  }
  const DELTA_PX: number = 5; // 用于补偿 gap 造成无法完全显示边框的问题
  const maxScroll =
    tagsContainerRef.value.scrollWidth -
    tagsContainerRef.value.clientWidth +
    DELTA_PX;
  const elapsed = Date.now() - start;
  const SPEED: number = 0.05;

  const shift = Math.min(SPEED * elapsed, maxScroll);

  tagsContainerRef.value.style.transform = `translateX(-${shift}px)`;

  if (shift < maxScroll) {
    requestAnimationFrame(scrollTagsContainer);
  } else {
    endTimeout = setTimeout(() => {
      if (!tagsContainerRef.value) return;
      const el = tagsContainerRef.value;

      const onFadeOutEnd = () => {
        el.removeEventListener("transitionend", onFadeOutEnd);
        el.style.transition = "none";
        el.style.transform = "";

        const onFadeInEnd = () => {
          el.removeEventListener("transitionend", onFadeInEnd);
          el.style.transition = "none";
          start = null;
          startTimeout = setTimeout(() => {
            requestAnimationFrame(scrollTagsContainer);
          }, START_TIME_MS);
        };
        el.addEventListener("transitionend", onFadeInEnd);
        el.style.transition = "opacity 300ms";
        el.style.opacity = "1";
      };

      el.addEventListener("transitionend", onFadeOutEnd);
      el.style.transition = "opacity 300ms";
      el.style.opacity = "0";
    }, RESET_TIME_MS);
  }
};

watch(
  () => props.isExpanded,
  isExpanded => {
    if (startTimeout !== null) {
      clearTimeout(startTimeout);
      startTimeout = null;
    }
    if (endTimeout !== null) {
      clearTimeout(endTimeout);
      endTimeout = null;
    }
    start = null;
    if (isExpanded) {
      if (tagsContainerRef.value) {
        tagsContainerRef.value.style.transition = "transform 300ms";
        tagsContainerRef.value.style.transform = "";
      }
    } else {
      if (tagsContainerRef.value) {
        tagsContainerRef.value.style.transition = "none";
      }
      startTimeout = setTimeout(() => {
        requestAnimationFrame(scrollTagsContainer);
      }, 500);
    }
  }
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
      <div class="flex flex-col flex-1 min-w-[4ch]">
        <div class="flex flex-row gap-2 items-center mb-1">
          <p
            class="font-semibold text-base-content flex-1"
            :class="!isExpanded ? 'truncate' : 'whitespace-normal break-all'"
          >
            {{ item.title }}
          </p>
          <div
            class="overflow-hidden max-w-[70%] min-w-2"
            ref="tagsContainerParentRef"
          >
            <div class="flex gap-1 pr-12" ref="tagsContainerRef">
              <span
                v-for="tag in item.tags"
                :key="tag"
                class="badge badge-sm badge-outline"
              >
                {{ tag }}
              </span>
            </div>
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
            <Pen class="w-4" />
            编辑
          </button>
          <button
            class="btn btn-sm"
            :class="isCopied ? 'btn-success' : 'btn-primary'"
            @click.stop="$emit('copy', item.id, item.content)"
          >
            <Copy v-if="!isCopied" class="w-4" />
            <Check v-else />
            {{ isCopied ? "已复制" : "复制" }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
