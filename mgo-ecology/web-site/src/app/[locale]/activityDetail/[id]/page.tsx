import { EventDetailType, ParamsLocale } from '@/interface';
import dayjs from 'dayjs'
import { $ssrGet } from '@/utils/request'
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import '@/styles/acdetail.scss'
import { getLangType } from '@/utils'
import Back from './_components/back'
import type { Metadata } from "next";
import { MetaDataType } from '@/interface'

export async function generateMetadata({ params: { locale, id } }: ParamsLocale): Promise<Metadata> {
  const res = await $ssrGet(`/eventSeo?language=${getLangType(locale)}&id=${id}`)
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

  const res = await $ssrGet(`/eventDetail?language=${getLangType(locale)}&id=${id}`)

  const eventDetail: EventDetailType = res.data.detail
  const type: number = res.data.type

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className='acdetail-style'>
        <div className='box1'>
          <div className='mb-5 start-center'>
            <Back />
          </div>
          <div className='b1-text1'>
            {eventDetail.title}
          </div>
        </div>
        <div className='box-px pt-10 md:pt-[120px] pb-10 md:pb-[180px]'>
          <div className='text-center'>
            <div className='text-white text-[48px] mb-[100px] hidden md:block'>
              {dayjs(eventDetail.startTime).format('YYYY-MM-DD')}
            </div>
          </div>
          <div className='between-start gap-[15px] md:gap-[80px] acd-flex1'>
            <div className='w-full md:w-[66%] flex-shrink-0'>
              <div className='relative w-full mb-[30px]'>
                <img src={eventDetail?.image} className="w-full object-cover block" alt="" />
              </div>
              <div className='text-white' dangerouslySetInnerHTML={{ __html: eventDetail.content }}></div>
            </div>
            <div className='pt-0 md:pt-[67px] flex-auto'>
              <div className='acd-titlebox center-center'>
                {
                  type == 0 ? t('t214') : type == 1 ? t('t215') : type == 2 ? t('t216') : ''
                }
              </div>
              <div className='text-gray83 text-[14px] md:text-lg mb-[10px] md:mb-5'>{t('t59')}</div>
              <div className='text-gray83 text-[14px] md:text-lg mb-[10px]'>
                {t('t60')}:
                <span className='text-white'> {dayjs(eventDetail.startTime).format('YYYY-MM-DD')}</span>
              </div>
              <div className='text-gray83 text-lgtext-[14px] md:text-lg mb-[10px] md:mb-10'>
                {t('t61')}:
                <span className='text-white'> {dayjs(eventDetail.endTime).format('YYYY-MM-DD')}</span>
              </div>
              <div className='text-gray83 text-[14px] md:text-lg mb-[10px] md:mb-5'>{t('t62')}</div>
              <div className='text-gray83 text-[14px] md:text-lg mb-10'>
                <span className='text-white'>{eventDetail?.location}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </TranslationsProvider>
  )
}
