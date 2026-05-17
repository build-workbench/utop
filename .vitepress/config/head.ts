/**
 * HTML head 标签配置
 */
import type { HeadConfig } from 'vitepress'
import { BASE_PATH } from './deploy'

export const head: HeadConfig[] = [
  ['meta', { name: 'viewport', content: 'width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no' }],
  ['meta', { name: 'theme-color', content: '#f59e0b' }],
  ['meta', { name: 'msapplication-TileColor', content: '#f59e0b' }],
  ['meta', { name: 'apple-mobile-web-app-capable', content: 'yes' }],
  ['meta', { name: 'apple-mobile-web-app-status-bar-style', content: 'black-translucent' }],
  ['meta', { name: 'apple-mobile-web-app-title', content: 'BYOT' }],
  ['meta', { name: 'msapplication-TileImage', content: `${BASE_PATH}logo.svg` }],

  ['meta', { name: 'robots', content: 'index, follow' }],
  ['meta', { name: 'referrer', content: 'no-referrer-when-downgrade' }],

  ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
  ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
  ['link', { rel: 'dns-prefetch', href: 'https://github.com' }],
  ['link', { rel: 'dns-prefetch', href: 'https://lessup.github.io' }],

  ['link', { rel: 'icon', type: 'image/svg+xml', href: `${BASE_PATH}logo.svg` }],
  ['link', { rel: 'mask-icon', href: `${BASE_PATH}logo.svg`, color: '#f59e0b' }],
  ['link', { rel: 'manifest', href: `${BASE_PATH}manifest.json` }],

  ['meta', { name: 'author', content: 'LessUp' }],
  ['meta', { name: 'copyright', content: 'Copyright 2025-2026 LessUp. All rights reserved.' }],
]
