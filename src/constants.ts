export const MAX_TITLE_LENGTH = 50;
export const MAX_TAG_LENGTH = 20;

export interface ProviderDef {
  value: string;
  label: string;
  models: string[];
  defaultUrl: string;
}

export const PROVIDERS: ProviderDef[] = [
  {
    value: "deepseek",
    label: "DeepSeek",
    models: ["deepseek-v4-pro", "deepseek-v4-flash"],
    defaultUrl: "https://api.deepseek.com/chat/completions",
  },
  {
    value: "custom",
    label: "自定义提供商",
    models: [],
    defaultUrl: "",
  },
];
