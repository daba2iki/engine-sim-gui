import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  // 開発時のみ有効にする設定
  server: {
    port: 5173,
    host: 'localhost'
  },
  // 生産環境向けの最適化を無効化する (必要に応じて調整)
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vue: ['vue'],
          naive: ['naive-ui']
        }
      }
    }
  }
})