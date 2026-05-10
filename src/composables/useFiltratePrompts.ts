import { Prompt } from "@/types";
import { Ref, ref, watch } from "vue";
import { useFilterStore } from "@/composables/useFilterStore";
import Fuse from "fuse.js";

export const useFiltratePrompts = (originalPrompts: Ref<Prompt[] | null>) => {
  const { selectedTags, searchedTitle } = useFilterStore();
  const filtratedPrompts = ref<Prompt[] | null>(null);

  let fuse: Fuse<string> | null = null;

  const initFuse = () => {
    if (originalPrompts.value === null) return;
    fuse = new Fuse(originalPrompts.value.map(prompt => prompt.title));
  };

  const updated = () => {
    if (originalPrompts.value === null || fuse === null) return;
    const neverNullFuse = fuse;   // 虽然可以使用 ! 断言，但是这样更好一些

    filtratedPrompts.value = originalPrompts.value.filter(prompt => {
      if (selectedTags.value.length > 0 && !prompt.tags.some(tag => selectedTags.value.includes(tag))) {
        return false;
      }

      if (searchedTitle.value.trim()) {
        const results = neverNullFuse.search(searchedTitle.value).map(result => result.item);
        if (!results.includes(prompt.title)) return false;
      }

      return true;
    });
  };

  watch(originalPrompts, () => {
    initFuse();
    updated();
  }, { immediate: true, deep: true });
  watch(selectedTags, updated);
  watch(searchedTitle, updated);

  return {
    filtratedPrompts,
  };
};
