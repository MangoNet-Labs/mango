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

let lang = localStorage.getItem('suiswap:lang') ?? 'en'
localStorage.setItem('suiswap:lang', lang)

i18n.use(initReactI18next).init({
  resources,
  lng: lang,
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
