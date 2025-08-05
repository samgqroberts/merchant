import { defineConfig, mergeConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    wasm(),
  ],
});
