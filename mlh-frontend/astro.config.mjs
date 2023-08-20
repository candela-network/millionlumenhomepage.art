import { defineConfig } from 'astro/config';

import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
  site: "http://localhost:3000",
  output: "server",
  adapter: node({
    mode: "standalone"
  })
});
