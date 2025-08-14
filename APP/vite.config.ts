import uni from "@dcloudio/vite-plugin-uni";
import { defineConfig, UserConfig } from "vite";

export default defineConfig(async () => {
  return {
    plugins: [
      uni(),
      (await import("unocss/vite")).default(),
      (await import("@vitejs/plugin-vue-jsx")).default(),
    ],
    esbuild: {
      drop: ["console", "debugger"],
    },
  } satisfies UserConfig;
});
