import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "node:path";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  server: {
    fs: {
      allow: [
        path.resolve(__dirname, "index.html"),
        path.resolve(__dirname, "public"),
        path.resolve(__dirname, "src"),
        path.resolve(__dirname, "..", "..", "crates", "debugger-wasm", "pkg"),
      ],
    },
    proxy: {
      "/sonolus": "https://sonolus.sekai.best",
    },
  },
});
