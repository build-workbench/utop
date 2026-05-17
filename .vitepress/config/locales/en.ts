/**
 * 英文语言区域配置
 */
import type { LocaleConfig } from 'vitepress'

export const en: LocaleConfig[string] = {
  label: 'English',
  lang: 'en',
  title: 'Build Your Own Tools',
  titleTemplate: ':title | BYOT',
  description: 'Technical Whitepaper - Rust × Go Dual-Implementation Architecture Comparison',
  link: '/en/',
  themeConfig: {
    siteTitle: 'BYOT',
    outline: { level: [2, 3], label: 'On this page' },
    docFooter: { prev: 'Previous page', next: 'Next page' },
    returnToTopLabel: 'Return to top',
    sidebarMenuLabel: 'Menu',
    darkModeSwitchLabel: 'Appearance',
    lightModeSwitchTitle: 'Switch to light theme',
    darkModeSwitchTitle: 'Switch to dark theme',
    editLink: {
      pattern: 'https://github.com/LessUp/build-your-own-tools/edit/master/:path',
      text: 'Edit this page on GitHub'
    },
    lastUpdated: {
      text: 'Updated at',
      formatOptions: { dateStyle: 'short', timeStyle: 'short' }
    },
    footer: {
      message: 'Released under the <a href="https://github.com/LessUp/build-your-own-tools/blob/master/LICENSE">MIT OR Apache-2.0</a> License',
      copyright: `Copyright © 2025-${new Date().getFullYear()} <a href="https://github.com/LessUp">LessUp</a>`
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/LessUp/build-your-own-tools' },
    ],
    search: { provider: 'local' },
    nav: [
      { text: 'Whitepaper', link: '/en/whitepaper/', activeMatch: '/en/whitepaper/' },
      { text: 'Specifications', link: '/en/specs/', activeMatch: '/en/specs/' },
      { text: 'Comparison', link: '/en/comparison/', activeMatch: '/en/comparison/' },
      {
        text: 'Implementations',
        items: [
          { text: '🔧 dos2unix', link: '/en/dos2unix/' },
          { text: '📦 gzip', link: '/en/gzip/' },
          { text: '📊 htop', link: '/en/htop/' },
        ]
      },
      { text: 'Engineering', link: '/en/engineering/', activeMatch: '/en/engineering/' },
      {
        text: 'More',
        items: [
          { text: '📋 Changelog', link: '/en/CHANGELOG' },
          { text: '🚀 Getting Started', link: '/en/docs/setup/GETTING-STARTED' },
          { text: '⭐ GitHub', link: 'https://github.com/LessUp/build-your-own-tools' },
        ]
      }
    ],
    sidebar: {
      '/en/whitepaper/': [
        {
          text: 'Whitepaper',
          items: [
            { text: 'Overview', link: '/en/whitepaper/' },
            { text: 'Project Overview', link: '/en/whitepaper/overview' },
            { text: 'System Architecture', link: '/en/whitepaper/architecture' },
            { text: 'Design Decisions', link: '/en/whitepaper/decisions' },
            { text: 'Performance', link: '/en/whitepaper/performance' },
          ]
        },
      ],
      '/en/specs/': [
        {
          text: 'Specifications',
          items: [
            { text: 'Overview', link: '/en/specs/' },
            { text: 'OpenSpec Workflow', link: '/en/specs/openspec-workflow' },
            { text: 'dos2unix Spec', link: '/en/specs/dos2unix' },
            { text: 'gzip Spec', link: '/en/specs/gzip' },
            { text: 'htop Spec', link: '/en/specs/htop' },
          ]
        },
      ],
      '/en/comparison/': [
        {
          text: 'Comparison',
          items: [
            { text: 'Overview', link: '/en/comparison/' },
            { text: 'Memory Model', link: '/en/comparison/memory' },
            { text: 'Concurrency', link: '/en/comparison/concurrency' },
            { text: 'Error Handling', link: '/en/comparison/errors' },
            { text: 'Benchmarks', link: '/en/comparison/benchmarks' },
          ]
        },
      ],
      '/en/engineering/': [
        {
          text: 'Engineering',
          items: [
            { text: 'Overview', link: '/en/engineering/' },
            { text: 'AI Collaboration', link: '/en/engineering/ai-collaboration' },
            { text: 'CI/CD Design', link: '/en/engineering/cicd' },
            { text: 'Documentation Strategy', link: '/en/engineering/documentation' },
          ]
        },
      ],
      '/en/docs/': [
        {
          text: 'Quick Reference',
          items: [
            { text: 'Getting Started', link: '/en/docs/setup/GETTING-STARTED' },
            { text: 'Architecture', link: '/en/docs/architecture/ARCHITECTURE' },
            { text: 'Comparison', link: '/en/docs/tutorials/COMPARISON' },
          ]
        },
      ],
      '/en/dos2unix/': [
        { text: 'Overview', link: '/en/dos2unix/' },
        { text: '← Back to Home', link: '/en/' },
      ],
      '/en/gzip/': [
        { text: 'Overview', link: '/en/gzip/' },
        { text: 'Go Implementation', link: '/en/gzip/go/' },
        { text: 'Rust Implementation', link: '/en/gzip/rust/' },
        { text: '← Back to Home', link: '/en/' },
      ],
      '/en/htop/': [
        { text: 'Overview', link: '/en/htop/' },
        { text: 'Unix Rust', link: '/en/htop/unix/rust/' },
        { text: 'Unix Go', link: '/en/htop/unix/go/' },
        { text: 'Windows Rust', link: '/en/htop/win/rust/' },
        { text: 'Windows Go', link: '/en/htop/win/go/' },
        { text: '← Back to Home', link: '/en/' },
      ],
    },
  }
}
