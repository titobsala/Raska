import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@": resolve(__dirname, "./src"),
    },
  },
  server: {
    port: 5173,
    proxy: {
      // Proxy API requests to the Rust backend during development
      '/api': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      },
      '/ws': {
        target: 'ws://127.0.0.1:3000',
        ws: true,
      },
      '/health': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      }
    }
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
    // Ensure compatibility with the Rust binary embedding
    rollupOptions: {
      output: {
        manualChunks: undefined,
      }
    }
  }
})