import { ref } from "vue";

const searchedTitle = ref("");
const selectedTags = ref<string[]>([]);

export const useFilterStore = () => ({
  searchedTitle,
  selectedTags,
});
