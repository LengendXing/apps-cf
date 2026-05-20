import { createI18n } from 'vue-i18n'
import zh from '../i18n/zh'
import en from '../i18n/en'

const i18n = createI18n({
  legacy: false,
  locale: localStorage.getItem('locale') || 'zh',
  fallbackLocale: 'en',
  messages: { zh, en },
})

export default i18n
