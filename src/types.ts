export interface Prompt {
  id: string;
  title: string;
  tip: string;
  content: string;
  tags: string[];
}

export type PromptData = Omit<Prompt, "id">;
