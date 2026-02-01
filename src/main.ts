import { createApp } from "vue";
import "@/style.css";
import App from "./App.vue";

const app = createApp(App);

const root = app.mount("#app");
const rootEl = root.$el as HTMLElement;

rootEl.addEventListener("contextmenu", event => {
  const target = event.target as HTMLElement | null;
  if (target && target.closest("input, textarea, [contenteditable='true']")) {
    return;
  }
  event.preventDefault();
});
