<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { useToast } from "@/composables/useToast";

const { toasts, removeToast } = useToast();

const alertClassMap = {
  info: "alert-info",
  success: "alert-success",
  warning: "alert-warning",
  error: "alert-error",
};

const activeTeleportTarget = ref("body");

const updateTeleportTarget = () => {
  const activeDialog =
    document.querySelector<HTMLDialogElement>("dialog[open]");
  activeTeleportTarget.value = activeDialog ? "dialog[open]" : "body";
};

const hasToasts = computed(() => toasts.value.length > 0);

let observer: MutationObserver | null = null;

onMounted(() => {
  updateTeleportTarget();
  observer = new MutationObserver(() => {
    updateTeleportTarget();
  });
  observer.observe(document.body, {
    attributes: true,
    childList: true,
    subtree: true,
    attributeFilter: ["open"],
  });
});

onBeforeUnmount(() => {
  observer?.disconnect();
  observer = null;
});
</script>

<template>
  <Teleport v-if="hasToasts" :to="activeTeleportTarget">
    <div class="app-toast toast toast-top toast-end">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="alert shadow-lg min-w-48"
        :class="[
          alertClassMap[toast.level],
          { 'text-white': toast.level === 'error' },
        ]"
      >
        <span>{{ toast.message }}</span>
        <button class="btn btn-ghost btn-xs ml-auto" @click="removeToast(toast.id)">
          关闭
        </button>
      </div>
    </div>
  </Teleport>
</template>

<style scoped lang="css">
.app-toast {
  z-index: 1000 !important;
}
</style>
