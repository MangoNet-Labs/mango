import { ParamsLocale } from '@/interface';
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import clsx from 'clsx'
import { Chiq_Reduced_Bold } from '@/font/font'
import '@/styles/technology.scss'
import TechnologyBox1Btn from './_components/technologyBox1Btn'
import { $isrGet } from '@/utils/request'
import { getLangType } from '@/utils'
import { NetworkType } from '@/interface';
import AdvantagesBox from './_components/advantagesBox'
import Swiper from './_components/swiper'
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/technology', locale)
}

const i18nNamespaces = ['common'];

export default async function page({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);
  const res = await $isrGet(`/network?language=${getLangType(locale)}`)
  const Advantages: NetworkType[] = res.data['3']
  const Agreement: NetworkType[] = res.data['4']
  const Model: NetworkType[] = res.data['0']
  const System: NetworkType[] = res.data['2']
  const move: NetworkType[] = res.data['1']

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className='technology-style'>
        <div className='box1'>
          <div className={
            clsx('b1-text1', Chiq_Reduced_Bold.className)
          }>Mango Network</div>
          <div className='pt-[10px] pb-[10px] md:pb-[110px] text-[12px] md:text-2xl text-white'>{t('t175')}</div>
          <div>
            <TechnologyBox1Btn />
          </div>
        </div>
        <div className='pt-10 md:pt-[180px] box-px'>
          <div className='text-center text-[18px] md:text-[48px] font-medium mb-[10px] md:mb-[100px] text-white'>
            {t('t176')}
          </div>
          <div className='tec-grid1'>
            {
              Model.map((item, i) => (
                <div className='tecg1-item between-center gap-[10px] md:gap-[30px]' key={item.ID}>
                  <div className='tecg1-item-img center-center' style={{
                    backgroundImage: `url('${i == 0 ? '/img/img65.png' : i == 1 ? '/img/img67.png' : i == 2 ? '/img/img69.png' : i == 3 ? '/img/img71.png' : ''}')`
                  }}>
                    <img src={item.image} className="w-[22px] md:w-[44px]" alt="" />
                  </div>
                  <div className="flex-auto">
                    <div className="text-[14px] md:text-2xl font-semibold text-white mb-[10px] md:mb-5">{item.title}</div>
                    <div className="text-grayb2 text-[12px] md:text-base leading-[2]">
                      {item.introduction}
                    </div>
                  </div>
                </div>
              ))
            }
          </div>
        </div>
        <div className="pt-10 md:pt-[160px] box-px">
          <div className="between-center gap-[10px] md:gap-[60px] tec-flex1">
            <div className="w-full md:w-[40%] flex-shrink-0">
              <div className="text-center md:text-left text-[18px] md:text-[48px] text-white font-semibold mb-[5px] md:mb-10">{t('t177')}</div>
              <div className="leading-[2] text-[12px] md:text-base text-white text-center md:text-left">
                {t('t178')}
              </div>
            </div>
            <div className="flex-auto">
              {
                move.map(item => (
                  <div className="between-start gap-[10px] md:gap-[30px] tec-add-1" key={item.ID}>
                    <div className="pt-0 md:pt-8 w-9 md:w-[56px] flex-shrink-0 tec-add-2 center-center">
                      <img src={item.image} className="w-[22px] md:w-full" alt="" />
                    </div>
                    <div className="tec-bb flex-auto">
                      <div className="text-[14px] md:text-[32px] text-white font-semibold mb-[5px] md:mb-[10px]">{item.title}
                      </div>
                      <div className="text-[12px] md:text-base text-white leading-[2]">
                        {item.introduction}
                      </div>
                    </div>
                  </div>
                ))
              }
            </div>
          </div>
        </div>
        <div className='pt-[26px] md:pt-[180px] box-px'>
          <div className="text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">
              {t('t179')}
            </div>
          </div>
          <Swiper System={System} />
        </div>
        <AdvantagesBox Advantages={Advantages} />
        <div className="pt-10 md:pt-[180px] box-px">
          <div className="text-[18px] md:text-[48px] text-white pb-[10px] md:pb-[110px] text-center font-semibold">
            {t('t182')}
          </div>
          <div className="tec-grid3">
            {
              Agreement.map(item => (
                <div className='tec-add-3' key={item.ID}>
                  <div className="start-center gap-[10px] md:gap-5 mb-[5px] md:mb-[30px]">
                    <img src={item.image} className="w-[30px] md:w-[56px] flex-shrink-0" alt="" />
                    <div className="text-[14px] md:text-[28px] text-white">{item.title}</div>
                  </div>
                  <div className="text-[12px] md:text-base text-grayce leading-[2]">
                    {item.introduction}
                  </div>
                </div>
              ))
            }
          </div>
        </div>
        <div className="h-[40px] md:h-[100px]"></div>
      </div>
    </TranslationsProvider>
  )
}
