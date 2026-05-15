export interface Prompt {
  id: string;
  title: string;
  tip: string;
  content: string;
  tags: string[];
}

export type PromptData = Omit<Prompt, "id">;

export interface ModelConfig {
  api_key: string;
  provider: string;
  model: string;
  base_url: string;
}
