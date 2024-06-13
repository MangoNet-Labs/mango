import { ParamsLocale } from '@/interface';
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import '@/styles/bolgList.scss'
import Top from './_components/top'
import List from './_components/list'
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/blogList', locale)
}

const i18nNamespaces = ['common'];

export default async function page({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className="bloglist-style">
        <Top />
        <div className='box2 box-px'>
          <div className='b2-title'>{t('t94')}</div>
          <List />
        </div>
      </div>
    </TranslationsProvider>
  )
}
