import { ParamsLocale } from '@/interface';
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import clsx from 'clsx'
import { Chiq_Reduced_Bold } from '@/font/font'
import '@/styles/activity.scss'
import List from './_components/list'
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/activity', locale)
}

const i18nNamespaces = ['common'];

export default async function page({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className='activity-style'>
        <div className={
          clsx('box1', Chiq_Reduced_Bold.className)
        }>
          {t('t14')}
        </div>
        <div className='pt-10 md:pt-[120px] pb-[15px] md:pb-[60px] text-center text-[18px] md:text-[48px] text-white'>{t('t63')}</div>
        <List />
      </div>
    </TranslationsProvider>
  )
}
