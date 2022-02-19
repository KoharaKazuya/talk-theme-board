import App from "./app.js";

const app = new App();

window.__TAURI__.globalShortcut.register("Cmd+Shift+k", () => {
  app.prev();
});
window.__TAURI__.globalShortcut.register("Cmd+Shift+j", () => {
  app.next();
});

window.__TAURI__.globalShortcut.register("Cmd+Shift+o", async () => {
  const content = await window.__TAURI__.clipboard.readText();

  const themes = await window.__TAURI__.invoke("load_text", { content });
  app.start(themes);
});

window.__TAURI__.event.listen("tauri://file-drop", async (event) => {
  const paths = event.payload;
  if (!(Array.isArray(paths) && paths.length > 0)) return;
  const path = paths[0];

  const themes = await window.__TAURI__.invoke("load_file", { path });
  app.start(themes);
});

window.addEventListener(
  "click",
  (event) => {
    const target = event.target;
    if (
      !(target instanceof HTMLElement && target.tagName.toLowerCase() === "a")
    )
      return;

    event.preventDefault();
    event.stopPropagation();

    const href = target.getAttribute("href");
    window.__TAURI__.shell.open(href);
  },
  { capture: true }
);

window.__TAURI__.globalShortcut.register("Cmd+Shift+Ctrl+c", () => {
  window.__TAURI__.process.exit();
});
