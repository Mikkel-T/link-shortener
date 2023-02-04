import svelte from "@astrojs/svelte";
import tailwind from "@astrojs/tailwind";
import { defineConfig } from "astro/config";
import Icons from "unplugin-icons/vite";

// https://astro.build/config
export default defineConfig({
  site: "https://admin-link.mikkel-t.com",
  integrations: [svelte(), tailwind()],
  vite: {
    plugins: [Icons({ compiler: "svelte" })],
  },
});
