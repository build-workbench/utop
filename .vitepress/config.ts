import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'
import { generateSitemap as sitemap } from 'sitemap-ts'

import { resolveBasePath, SITE_URL } from './config/deploy'
import { head } from './config/head'
import { vite } from './config/vite'
import { zhCN, en } from './config/locales'

const base = resolveBasePath()

export default withMermaid(defineConfig({
  // ===========================================================================
  // 基础配置
  // ===========================================================================
  title: 'Build Your Own Tools',
  description: '系统编程技术白皮书 - Rust × Go 双实现架构对比研究',
  base,
  cleanUrls: true,
  lastUpdated: true,
  ignoreDeadLinks: true,
  srcExclude: [
    'AGENTS.md',
    'CLAUDE.md',
    'release-notes.md',
    'openspec/**',
    '.opencode/**',
    '.claude/**',
    '.github/**',
  ],

  // ===========================================================================
  // 国际化配置
  // ===========================================================================
  locales: { root: zhCN, en },

  // ===========================================================================
  // 头信息 - SEO 和 PWA 支持
  // ===========================================================================
  head,

  // ===========================================================================
  // Markdown 配置
  // ===========================================================================
  markdown: {
    lineNumbers: true,
    anchor: {
      level: [1, 2, 3, 4]
    },
    toc: {
      level: [2, 3],
    },
    languageAlias: {
      toml: 'ini',
      rs: 'rust',
      golang: 'go',
      bash: 'shell',
      zsh: 'shell',
    }
  },

  // ===========================================================================
  // Mermaid 配置
  // ===========================================================================
  mermaid: {
    // 参考: https://mermaid.js.org/config/theming.html
  },

  // ===========================================================================
  // 构建钩子 - 生成 Sitemap
  // ===========================================================================
  buildEnd: async (siteConfig) => {
    await sitemap({
      hostname: SITE_URL,
      outDir: siteConfig.outDir,
      exclude: ['/404.html', '/search.html'],
      changefreq: 'weekly',
      priority: 0.8,
      lastmod: new Date(),
    })
  },

  // ===========================================================================
  // Vite 配置
  // ===========================================================================
  vite,

  // ===========================================================================
  // Vue 配置
  // ===========================================================================
  vue: {
    template: {
      compilerOptions: {
        isCustomElement: (tag) => tag.includes('-'),
      },
    },
  },
}))
