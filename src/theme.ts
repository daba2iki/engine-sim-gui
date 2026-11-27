/**
 * Single source of truth for the yellow palette (JS side).
 * The CSS-side variables in style.css mirror these exact values.
 *
 * NOTE: Naive UI's themeOverrides MUST receive literal, parseable colors —
 * it derives hover/pressed shades at runtime (seemly/rgba) and throws on
 * "var(--...)" strings, which crashes the whole app at mount.
 */
export const palette = {
  accent: '#FFC81E',
  accentHover: '#FFD54A',
  accentActive: '#E6B000',
  accentText: '#1A1A1A',
  selectedBg: '#FFF8E0',
  selectedText: '#B38600',
  bg: '#FFFFFF',
  sidebarBg: '#F7F7F5',
  cardBg: '#FAFAFA',
  textPrimary: '#333333',
  textSecondary: '#999999',
  border: '#EEEEEE',
  white: '#FFFFFF',
  focusRing: 'rgba(255, 200, 30, 0.20)'
} as const
