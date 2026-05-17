/**
 * Vite 构建配置
 */
import { resolve } from 'path'
import llmstxt from 'vitepress-plugin-llms'

export const vite = {
  plugins: [llmstxt()],
  build: {
    chunkSizeWarningLimit: 1000,
    rollupOptions: {
      output: {
        assetFileNames: (assetInfo: { name?: string }) => {
          const info = (assetInfo.name ?? 'asset').split('.')
          const ext = info[info.length - 1]
          if (/png|jpe?g|svg|gif|tiff|bmp|ico/i.test(ext)) {
            return `assets/images/[name][extname]`
          }
          return `assets/[name]-[hash][extname]`
        },
      },
    },
  },
  optimizeDeps: {
    include: ['vue', '@vue/runtime-dom'],
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, '..'),
    },
  },
  server: {
    port: 5173,
    strictPort: false,
    open: true,
  },
  preview: {
    port: 4173,
    strictPort: false,
  },
}
