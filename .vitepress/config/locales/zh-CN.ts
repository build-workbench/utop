/**
 * 中文语言区域配置
 */
import type { LocaleConfig } from 'vitepress'

export const zhCN: LocaleConfig[string] = {
  label: '简体中文',
  lang: 'zh-CN',
  title: 'Build Your Own Tools',
  titleTemplate: ':title | BYOT',
  description: '系统编程技术白皮书 - Rust × Go 双实现架构对比研究',
  themeConfig: {
    siteTitle: 'BYOT',
    outline: { level: [2, 3], label: '目录' },
    docFooter: { prev: '上一页', next: '下一页' },
    returnToTopLabel: '返回顶部',
    sidebarMenuLabel: '菜单',
    darkModeSwitchLabel: '主题',
    lightModeSwitchTitle: '切换到浅色模式',
    darkModeSwitchTitle: '切换到深色模式',
    editLink: {
      pattern: 'https://github.com/LessUp/build-your-own-tools/edit/master/:path',
      text: '在 GitHub 上编辑此页'
    },
    lastUpdated: {
      text: '最后更新于',
      formatOptions: { dateStyle: 'short', timeStyle: 'short' }
    },
    footer: {
      message: '基于 <a href="https://github.com/LessUp/build-your-own-tools/blob/master/LICENSE">MIT OR Apache-2.0</a> 许可发布',
      copyright: `Copyright © 2025-${new Date().getFullYear()} <a href="https://github.com/LessUp">LessUp</a>`
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/LessUp/build-your-own-tools' },
    ],
    search: {
      provider: 'local',
      options: {
        locales: {
          root: {
            translations: {
              button: { buttonText: '搜索文档', buttonAriaLabel: '搜索文档' },
              modal: {
                noResultsText: '无法找到相关结果',
                resetButtonTitle: '清除查询条件',
                footer: { selectText: '选择', navigateText: '切换', closeText: '关闭' }
              }
            }
          }
        }
      }
    },
    nav: [
      { text: '白皮书', link: '/whitepaper/', activeMatch: '/whitepaper/' },
      { text: '技术规范', link: '/specs/', activeMatch: '/specs/' },
      { text: '对比研究', link: '/comparison/', activeMatch: '/comparison/' },
      {
        text: '工具实现',
        items: [
          { text: '🔧 dos2unix', link: '/dos2unix/' },
          { text: '📦 gzip', link: '/gzip/' },
          { text: '📊 htop', link: '/htop/' },
        ]
      },
      { text: '工程实践', link: '/engineering/', activeMatch: '/engineering/' },
      {
        text: '更多',
        items: [
          { text: '📋 变更日志', link: '/CHANGELOG' },
          { text: '🚀 快速开始', link: '/docs/setup/GETTING-STARTED' },
          { text: '⭐ GitHub', link: 'https://github.com/LessUp/build-your-own-tools' },
        ]
      }
    ],
    sidebar: {
      '/whitepaper/': [
        {
          text: '白皮书',
          items: [
            { text: '概览', link: '/whitepaper/' },
            { text: '项目概览', link: '/whitepaper/overview' },
            { text: '系统架构', link: '/whitepaper/architecture' },
            { text: '设计决策', link: '/whitepaper/decisions' },
            { text: '性能分析', link: '/whitepaper/performance' },
          ]
        },
      ],
      '/specs/': [
        {
          text: '技术规范',
          items: [
            { text: '概览', link: '/specs/' },
            { text: 'OpenSpec 工作流', link: '/specs/openspec-workflow' },
            { text: 'dos2unix 规范', link: '/specs/dos2unix' },
            { text: 'gzip 规范', link: '/specs/gzip' },
            { text: 'htop 规范', link: '/specs/htop' },
          ]
        },
      ],
      '/comparison/': [
        {
          text: '对比研究',
          items: [
            { text: '概览', link: '/comparison/' },
            { text: '内存模型', link: '/comparison/memory' },
            { text: '并发模型', link: '/comparison/concurrency' },
            { text: '错误处理', link: '/comparison/errors' },
            { text: '性能基准', link: '/comparison/benchmarks' },
          ]
        },
      ],
      '/engineering/': [
        {
          text: '工程实践',
          items: [
            { text: '概览', link: '/engineering/' },
            { text: 'AI 协作指南', link: '/engineering/ai-collaboration' },
            { text: 'CI/CD 设计', link: '/engineering/cicd' },
            { text: '文档策略', link: '/engineering/documentation' },
          ]
        },
      ],
      '/docs/': [
        {
          text: '快速参考',
          items: [
            { text: '快速开始', link: '/docs/setup/GETTING-STARTED' },
            { text: '架构指南', link: '/docs/architecture/ARCHITECTURE' },
            { text: '语言对比', link: '/docs/tutorials/COMPARISON' },
          ]
        },
      ],
      '/dos2unix/': [
        { text: '概览', link: '/dos2unix/' },
        { text: '← 返回首页', link: '/' },
      ],
      '/gzip/': [
        { text: '概览', link: '/gzip/' },
        { text: 'Go 实现', link: '/gzip/go/' },
        { text: 'Rust 实现', link: '/gzip/rust/' },
        { text: '← 返回首页', link: '/' },
      ],
      '/htop/': [
        { text: '概览', link: '/htop/' },
        { text: 'Unix Rust', link: '/htop/unix/rust/' },
        { text: 'Unix Go', link: '/htop/unix/go/' },
        { text: 'Windows Rust', link: '/htop/win/rust/' },
        { text: 'Windows Go', link: '/htop/win/go/' },
        { text: '← 返回首页', link: '/' },
      ],
    },
  }
}
