import { useData } from 'vitepress'
import { translations, type TranslationKey } from '../i18n/translations'

// 定义支持的语言类型
type SupportedLang = 'zh-CN' | 'en'

/**
 * 翻译组合式函数
 * 根据当前语言返回对应的翻译文本
 *
 * @example
 * ```vue
 * <script setup>
 * import { useTranslation } from '../composables/useTranslation'
 * const { t } = useTranslation()
 * </script>
 *
 * <template>
 *   <button :aria-label="t('returnToTop')">↑</button>
 * </template>
 * ```
 */
export function useTranslation() {
  const { lang } = useData()

  /**
   * 获取翻译文本
   * @param key - 翻译键
   * @returns 当前语言对应的翻译文本
   */
  function t(key: TranslationKey): string {
    // VitePress 的 root locale 对应中文
    const langKey: SupportedLang = lang.value === 'root' ? 'zh-CN' : (lang.value as SupportedLang) || 'en'
    return translations[langKey]?.[key] || translations['en'][key] || key
  }

  return { t }
}
