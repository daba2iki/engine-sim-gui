import { defineConfig, type Plugin } from 'vite'
import vue from '@vitejs/plugin-vue'
import fs from 'node:fs'
import path from 'node:path'

/** Debug probe: the webview POSTs boot diagnostics here; we append to boot-log.txt */
function bootLogPlugin(): Plugin {
  return {
    name: 'boot-log',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (req.url === '/__bootlog') {
          let body = ''
          req.on('data', (c) => (body += c))
          req.on('end', () => {
            fs.appendFileSync(
              path.resolve(process.cwd(), 'boot-log.txt'),
              `[${req.headers['x-ts'] || 'log'}] ${body}\n`
            )
            res.setHeader('Content-Type', 'text/plain')
            res.end('ok')
          })
          return
        }
        next()
      })
    },
  }
}

export default defineConfig({
  plugins: [vue(), bootLogPlugin()],
  clearScreen: false,
  server: {
    host: '127.0.0.1',
    port: 1420,
    strictPort: true,
  },
  resolve: {
    alias: {
      '@': '/src'
    }
  }
})
