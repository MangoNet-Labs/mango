import { ParamsLocale } from '@/interface';
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import { $isrGet } from '@/utils/request'
import '@/styles/brand.scss'
import Download from './_components/download'
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/brand', locale)
}
const i18nNamespaces = ['common'];

export default async function page({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);
  const res = await $isrGet('/getBrands')
  const imageList: any = res.data.image
  const zip: string = res.data.zip

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className='brand-style box-px pt-5 pb-10 text-white'>
        <div className="text-center font-semibold text-[32px] mb-8">{t('t234')}</div>
        <Download imageList={imageList} zip={zip} />
      </div>
    </TranslationsProvider>
  )
}
