import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Vitest shares this config with the app. The `test` block was missing
// entirely: `pnpm test` existed in package.json and vitest/RTL/jsdom were all
// installed, but there was no jsdom environment, no globals, and — critically —
// `@vitejs/plugin-react` was never registered, so any `.tsx` test would have
// failed to transform JSX rather than fail for a real reason.
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:9097',
        ws: true,
      },
    },
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/test/setup.ts'],
    include: ['src/**/*.{test,spec}.{ts,tsx}'],
    restoreMocks: true,
  },
});
