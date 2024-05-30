import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import enUS from './en.json';
import zhCN from './zh.json';

const resources = {
  'en': {
    translation: enUS,
  },
  'zh': {
    translation: zhCN,
  },
};

let lang = localStorage.getItem('mgoswap:lang') ?? 'en'
localStorage.setItem('mgoswap:lang', lang)

i18n.use(initReactI18next).init({
  resources,
  lng: lang,
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
