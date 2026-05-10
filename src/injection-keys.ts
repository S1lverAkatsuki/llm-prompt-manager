import type { InjectionKey, Ref } from "vue";
import type { Prompt } from "@/types";

export const itemsKey: InjectionKey<Ref<Prompt[] | null>> = Symbol("items");

export const tagsKey: InjectionKey<{
  tags: Ref<string[]>;
  addTag: (tag: string) => Promise<void>;
  removeTag: (tag: string) => Promise<void>;
  refresh: () => Promise<void>;
}> = Symbol("tags");
