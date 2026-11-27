/**
 * Boot probe — loaded BEFORE main.ts to diagnose the blank Tauri webview.
 * Sends diagnostics to the Vite dev server endpoint /__bootlog (see vite.config.ts),
 * which appends them to boot-log.txt. Deliberately dependency-free.
 */
const lines: string[] = [
  `probe:start ${new Date().toISOString()}`,
  `  href: ${location.href}`,
  `  UA: ${navigator.userAgent}`,
  `  hasTauriInternals: ${'__TAURI_INTERNALS__' in window}`,
]

function send(tag: string) {
  try {
    void fetch('/__bootlog', {
      method: 'POST',
      headers: { 'content-type': 'text/plain', 'x-ts': tag },
      body: lines.join('\n'),
    })
  } catch {
    /* nothing else we can do */
  }
}

window.addEventListener('error', (e: ErrorEvent) => {
  lines.push(`ERROR @${e.filename}:${e.lineno}: ${e.message}`)
  const stack = (e.error as Error | null)?.stack
  if (stack) lines.push(String(stack).split('\n').slice(0, 8).join(' | '))
  send('error')
})

window.addEventListener('unhandledrejection', (e: PromiseRejectionEvent) => {
  const r = e.reason as { stack?: string; message?: string } | string
  const text = typeof r === 'string' ? r : r?.stack || r?.message || String(r)
  lines.push(`REJECTION: ${text}`)
  send('rejection')
})

setTimeout(() => {
  const app = document.querySelector('#app')
  lines.push(
    `T+2s: #app ${app ? `children=${app.childElementCount}, html=${app.innerHTML.length}` : 'MISSING'}, body children=${document.body.childElementCount}`
  )
  send('t2s')
}, 2000)

send('start')
