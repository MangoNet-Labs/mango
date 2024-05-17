import { useTranslation } from 'react-i18next';

export function useLanguage() {
  const { t, i18n } = useTranslation();
  const changeLang = (key: string) => {
    i18n.changeLanguage(key)
    localStorage.setItem('mgoswap:lang', key)
  }
  return {
    changeLang,
    t,
    lang: i18n.language
  }
}