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
    value: "openai",
    label: "OpenAI",
    models: ["gpt-4o", "gpt-4o-mini", "gpt-4.1", "o4-mini"],
    defaultUrl: "",
  },
  {
    value: "anthropic",
    label: "Anthropic",
    models: [
      "claude-sonnet-4-20250514",
      "claude-opus-4-20250514",
      "claude-3-5-haiku-20241022",
    ],
    defaultUrl: "",
  },
  {
    value: "google",
    label: "Google AI",
    models: ["gemini-2.5-flash", "gemini-2.5-pro"],
    defaultUrl: "",
  },
  {
    value: "custom",
    label: "自定义提供商",
    models: [],
    defaultUrl: "",
  },
];
