/**
 * 组件级翻译配置
 * 用于 Vue 组件中需要国际化的文本
 */
export const translations = {
  'zh-CN': {
    copy: '复制',
    copied: '已复制!',
    returnToTop: '返回顶部',
    copyFailed: '复制失败'
  },
  'en': {
    copy: 'Copy',
    copied: 'Copied!',
    returnToTop: 'Return to top',
    copyFailed: 'Copy failed'
  }
} as const

export type TranslationKey = keyof typeof translations['zh-CN']
