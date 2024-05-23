import Breadcrumb from './_components/breadcrumb'
import { ParamsLocale, MetaDataType } from '@/interface';
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import { BlogDetailType } from '@/interface';
import '@/styles/blogdetail.scss'
import dayjs from 'dayjs'
import { $isrGet } from '@/utils/request'
import { getLangType } from '@/utils'
import Change from './_components/change'
import { Suspense } from 'react'
import Recommend from './_components/recommend'
import type { Metadata } from "next";

export async function generateMetadata({ params: { locale, id } }: ParamsLocale): Promise<Metadata> {
  const res = await $isrGet(`/blogSeo?language=${getLangType(locale)}&id=${id}`)
  const result: MetaDataType = res.data
  return {
    title: result.title,
    description: result.introduction,
    keywords: result.keywords,
    openGraph: {
      title: result.title,
      description: result.introduction,
      images: result.image
    },
    twitter: {
      card: "summary_large_image",
      title: result.title,
      description: result.introduction,
      images: result.image
    }
  }
}

const i18nNamespaces = ['common'];

export default async function page({ params: { locale, id } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);
  const res = await $isrGet(`/blogDetail?language=${getLangType(locale)}&id=${id}`)
  const blogDetail: BlogDetailType = res.data

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className="blogdetail-style box-px">
        <div className='box1'>
          <Breadcrumb pageTitle={blogDetail.detail.title} />
        </div>
        <div className="box2 between-center">
          <img src={blogDetail.detail.avatar} className="b2-img" alt="" />
          <div className="b2-text1">{blogDetail.detail.name}</div>
          <div className="b2-text2" v-if="blogDetail?.detail">{dayjs(blogDetail.detail.releaseTime).format('YYYY-MM-DD')}</div>
        </div>
        <div className='box3'>
          <div className='b3-title'>{blogDetail.detail.title}</div>
          <div className='b3-content' dangerouslySetInnerHTML={{ __html: blogDetail.detail.content }}></div>
          <Suspense>
            <Change blogDetail={blogDetail} />
          </Suspense>
        </div>
        <Recommend />
      </div>
    </TranslationsProvider>
  )
}
