import preact from "@preact/preset-vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [preact(), tailwindcss()],
  build: {
    target: "es2015",
    cssMinify: "lightningcss",
    rollupOptions: {
      output: {
        manualChunks: {
          preact: ["preact"],
          "preact-iso": ["preact-iso"],
          clsx: ["clsx"],
        },
      },
    },
  },
});
