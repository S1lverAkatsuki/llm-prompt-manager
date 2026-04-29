import type { InjectionKey, Ref } from "vue";
import type { Prompt } from "./types";

export const itemsKey: InjectionKey<Ref<Prompt[] | null>> = Symbol("items");
