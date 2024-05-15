import { ParamsLocale } from '@/interface';
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import { $ssrGet } from '@/utils/request'
import { getLangType } from '@/utils'
import Index from './_components/index'
import type { MenuProps } from 'antd';
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/dapp', locale)
}

const i18nNamespaces = ['common'];

export default async function page({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);
  const [res, res2] = await Promise.all([
    (async () => {
      const result = await $ssrGet(`/classIfiCationList?language=${getLangType(locale)}`)
      return result
    })(),
    (async () => {
      const result = await $ssrGet(`/classList?language=${getLangType(locale)}`)
      return result
    })()
  ])

  const itemsList: MenuProps['items'] = res.data.map((e: any) => {
    return {
      label: e.title,
      key: e.ID.toString()
    }
  }) ?? []

  const categorys: MenuProps['items'] = res2.data.map((e: any) => {
    return {
      label: e.title,
      key: e.ID.toString()
    }
  }) ?? []

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <Index items={itemsList} categorys={categorys} />
    </TranslationsProvider>
  )
}
