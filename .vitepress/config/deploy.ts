/**
 * 部署配置 - 单一真相来源
 *
 * 所有与 GitHub Pages 部署相关的配置都从这里导出。
 * 修改部署路径只需更改此文件。
 */

/** GitHub 仓库名称（用作 base path） */
export const REPO_NAME = 'build-your-own-tools'

/** 完整基础路径（带前后斜杠） */
export const BASE_PATH = `/${REPO_NAME}/`

/** 完整站点 URL */
export const SITE_URL = `https://lessup.github.io${BASE_PATH}`

/** Sitemap URL */
export const SITEMAP_URL = `${SITE_URL}sitemap.xml`

/** 本地预览 URL（用于 Lighthouse CI） */
export const PREVIEW_URL = `http://localhost:4173${BASE_PATH}`

/**
 * 解析环境变量或返回默认基础路径
 * 支持 VITEPRESS_BASE 环境变量覆盖
 */
export function resolveBasePath(envValue?: string): string {
  const raw = envValue ?? process.env.VITEPRESS_BASE
  if (!raw) return BASE_PATH

  // 规范化：确保以 / 开头和结尾
  const normalized = raw.startsWith('/') ? raw : `/${raw}`
  return normalized.endsWith('/') ? normalized : `${normalized}/`
}
