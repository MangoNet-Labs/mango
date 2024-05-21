import '@/styles/apply.scss'
import { ParamsLocale } from '@/interface';
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import Index from './_components/index'
import { $ssrGet } from '@/utils/request'
import { getLangType } from '@/utils'
import { MainListType } from '@/interface';
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/apply', locale)
}

const i18nNamespaces = ['common'];

export default async function page({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);
  const res = await $ssrGet(`/mainList?language=${getLangType(locale)}`)

  const mainList: MainListType = res.data
  const categorys = mainList.main.map((ele, i) => ({
    value: i.toString(),
    label: ele
  }))
  const amountList = mainList.money.map((ele, i) => ({
    value: i.toString(),
    label: ele
  }))


  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className='apply-style box-px py-[25px] md:py-[100px]'>
        <div className="box1 mb-[15px] md:mb-[50px]">
          <div className="px-[15px] md:px-[50px] pt-[15px] md:pt-[50px] pb-[15px] md:pb-[30px] bb32">
            <div className="text-[24px] md:text-[48px] text-white font-medium mb-[10px] md:mb-5">{t('t72')}</div>
            <div className="text-[12px] md:text-2xl text-grayb2">{t('t73')}</div>
          </div>
          <div className="pt-[10px] md:pt-5 pb-[15px] md:pb-[30px] px-[15px] md:px-[50px] text-[12px] md:text-2xl">
            <span className="text-redff">* </span>
            <span className="text-white">{t('t74')}</span>
          </div>
        </div>
        <Index categorys={categorys} amountList={amountList} />
      </div>
    </TranslationsProvider>
  )
}