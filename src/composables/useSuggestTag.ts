import { Ref, ref, watch } from "vue";
import Fuse from "fuse.js";

export const useSuggestTag = (availableTags: Ref<string[]>, tagInput: Ref<string>) => {
  const suggestTags = ref<string[]>([]);

  let fuse = new Fuse(availableTags.value, {
    includeScore: true,
    threshold: 0.3
  });

  const updateSuggestTags = () => {
    if (tagInput.value.trim() === "") {
      suggestTags.value = [];
    } else {
      const results = fuse.search(tagInput.value);
      suggestTags.value = results.map((result) => result.item);
    }
  };

  watch(tagInput, updateSuggestTags, { immediate: true });

  watch(availableTags, () => {
    fuse = new Fuse(availableTags.value, {
      includeScore: true,
      threshold: 0.3
    });
    updateSuggestTags();
  });

  return {
    suggestTags
  };
};
