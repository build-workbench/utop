/**
 * 滚动揭示指令
 *
 * 元素进入视口时渐显动画
 */
import type { Directive } from 'vue'

export const scrollReveal: Directive = {
  mounted(el) {
    el.style.opacity = '0'
    el.style.transform = 'translateY(20px)'
    el.style.transition = 'all 0.6s cubic-bezier(0.4, 0, 0.2, 1)'

    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          el.style.opacity = '1'
          el.style.transform = 'translateY(0)'
          observer.unobserve(el)
        }
      })
    }, { threshold: 0.1 })

    observer.observe(el)
  }
}
