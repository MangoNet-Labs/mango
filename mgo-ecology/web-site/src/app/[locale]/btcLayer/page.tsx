import { ParamsLocale } from '@/interface';
import TranslationsProvider from '@/components/TranslationsProvider';
import initTranslations from '@/app/i18n'
import '@/styles/btclayer.scss'
import clsx from 'clsx'
import { Chiq_Reduced_Bold } from '@/font/font'
import Chart1 from '@/app/[locale]/_components/chart1'
import Chart2 from '@/app/[locale]/_components/chart2'
import Image from 'next/image'
import { Progress } from 'antd';
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/btcLayer', locale)
}

const i18nNamespaces = ['common'];

export default async function page({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className='btclayer-style'>
        <div className="box1">
          <div className={
            clsx('b1-text1', Chiq_Reduced_Bold.className)
          }>
            {t('t239')}
          </div>
          {/* <div className="pt-[5px] md:pt-[10px] pb-[15px] md:pb-[110px] text-[12px] md:text-2xl text-white">
            A more optimal BTC-native L2 bridging solution
          </div> */}
          <div className="pt-[5px] md:pt-[10px] text-[12px] md:text-2xl text-white">
            {t('t240')}
          </div>
          {/* <ChainBox1Btn /> */}
        </div>
        <div className="box-px pt180">
          <div className="between-center gap-[15px] md:gap-[50px] cha-add-1">
            <div className="flex-auto">
              <div className="pb-[10px] md:pb-[25px] text-[14px] md:text-[32px] font-semibold">
                <span className="text-white">Mango Network</span>
              </div>
              <div className="pb-[15px] md:pb-[40px] text-[12px] md:text-[18px] font-medium bb32">
                <span className="text-green81">
                  {t('t241')}
                </span>
              </div>
              <div className="pt-[15px] md:pt-5 pb-[15px] md:pb-[30px] bb32 text-[12px] md:text-xl text-grayb2">
                <span className='add-greenround'></span>
                {t('t242')}
              </div>
              <div className="pt-[15px] md:pt-5 pb-[15px] md:pb-[30px] bb32 text-[12px] md:text-xl text-grayb2">
                <span className='add-greenround'></span>
                {t('t243')}
              </div>
              <div className="pt-[15px] md:pt-5 pb-[15px] md:pb-[30px] bb32 text-[12px] md:text-xl text-grayb2">
                <span className='add-greenround'></span>
                {t('t244')}
              </div>
            </div>
            <div className="w-full md:w-[45%] flex-shrink-0 center-center">
              <div className="w-[70%] md:w-full addmovebg">
                <img src="/img/m2bg.png" className="w-full" alt="" />
                <img src="/img/m2.png" className="addmovebg-img" alt="" />
              </div>
            </div>
          </div>
        </div>
        <div className='box4 box-px'>
          <div className="text-center text-white mb-[30px] md:mb-[74px]">
            <div className="text-[18px] md:text-[48px] font-semibold mb-2">{t('t155')}</div>
            <div className="text-[12px] md:text-xl text-gray83 md:text-white">{t('t156')}</div>
          </div>
          <div className='b4-grid'>
            <div className="py-[15px] md:py-5 px-[15px] md:px-[30px] b4-item">
              <div className="text-[14px] md:text-lg font-bold mb-[10px] md:mb-5 text-green81">{t('t157')}</div>
              <div className="b4chi-grid">
                <div>
                  <div className="text-[12px] md:text-sm text-green81">{t('t158')}</div>
                  <div className="text-white text-[14px] md:text-xl">60 K</div>
                </div>
                <div>
                  <div className="gap-[6px] start-center">
                    <span className="text-[12px] md:text-sm text-green81">{t('t159')}</span>
                    <Image src="/img/img28.png" alt="" width={12} height={12} />
                  </div>
                  <div className="text-white text-[14px] md:text-xl">297.45 K</div>
                </div>
              </div>
              <div className="between-center py-[10px] bbblack32">
                <div className="gap-[6px] start-center">
                  <span className="text-xs md:text-sm text-green81">{t('t160')}</span>
                  <Image src="/img/img28.png" alt="" width={12} height={12} />
                </div>
                <div className="text-white text-[14px] md:text-xl">750 MANGO</div>
              </div>
              <div className="pt-[10px] pb-1">
                <div className="between-center mb-[5px]">
                  <span className="text-xs md:text-sm text-green81">{t('t161')}</span>
                  <div className="text-white text-[14px] md:text-xl">10.12 K</div>
                </div>
                <div className="between-center">
                  <span className="text-xs md:text-sm text-green81">{t('t162')}</span>
                  <div className="text-white text-[14px] md:text-xl">30.76 M</div>
                </div>
              </div>
            </div>
            <div className="py-[15px] md:py-5 px-[15px] md:px-[30px] b4-item">
              <div className="text-[14px] md:text-lg font-bold mb-[10px] md:mb-5 text-green81">{t('t163')} 287</div>
              <div className="text-[12px] md:text-sm text-green81 mb-[10px] md:mb-5">{t('t164')}</div>
              <div className="text-[14px] md:text-lg font-bold mb-[5px] text-green81">{t('t165')}</div>
              <div className="mb-[5px] md:mb-5 hidden md:block">
                <Progress percent={30} trailColor="rgba(235, 235, 235, 1)" strokeColor="#81D8CF" size={['100%', 16]} />
              </div>
              <div className="mb-[5px] md:mb-5 block md:hidden">
                <Progress percent={30} trailColor="rgba(235, 235, 235, 1)" strokeColor="#81D8CF" size={['100%', 8]} />
              </div>
              <div className="between-center">
                <div className="gap-[6px] start-center">
                  <span className="text-[12px] md:text-sm text-green81">{t('t166')}</span>
                  <Image src="/img/img28.png" alt="" width={12} height={12} />
                </div>
                <div className="text-white text-[14px] md:text-xl">24,465,670</div>
              </div>
            </div>
            <div className="b4-item py-[15px] md:py-5 px-[15px] md:px-[30px]">
              <div className="text-[14px] md:text-lg font-bold mb-[10px] md:mb-5 text-green81">{t('t167')}</div>
              <div className="b4item-grid mb-[10px] md:mb-5">
                <div>
                  <div className="gap-[6px] start-center">
                    <span className="text-[12px] md:text-sm text-green81">{t('t168')}</span>
                    <Image src="/img/img28.png" alt="" width={12} height={12} />
                  </div>
                  <div className="text-white text-[14px] md:text-xl">23.8 M</div>
                </div>
                <div>
                  <div className="gap-[6px] start-center">
                    <span className="text-[12px] md:text-sm text-green81">{t('t169')}</span>
                    <Image src="/img/img28.png" alt="" width={12} height={12} />
                  </div>
                  <div className="text-white text-[14px] md:text-xl">1.33M</div>
                </div>
                <div>
                  <div className="gap-[6px] start-center">
                    <span className="text-[12px] md:text-sm text-green81">{t('t170')}</span>
                    <Image src="/img/img28.png" alt="" width={12} height={12} />
                  </div>
                  <div className="text-white text-[14px] md:text-xl">2.56M</div>
                </div>
              </div>
              <Chart1 />
            </div>
            <div className="b4-item py-[15px] md:py-5 px-[15px] md:px-[30px]">
              <div className="text-[14px] md:text-lg font-bold mb-[10px] md:mb-5 text-green81">{t('t171')}</div>
              <div className="b4item-grid2 mb-[10px] md:mb-5">
                <div>
                  <div className="gap-[6px] start-center">
                    <span className="text-[12px] md:text-sm text-green81">{t('t168')}</span>
                    <Image src="/img/img28.png" alt="" width={12} height={12} />
                  </div>
                  <div className="text-white text-[14px] md:text-xl">9.58 M</div>
                </div>
                <div>
                  <div className="gap-[6px] start-center">
                    <span className="text-[12px] md:text-sm text-green81">{t('t172')}</span>
                    <Image src="/img/img28.png" alt="" width={12} height={12} />
                  </div>
                  <div className="text-white text-[14px] md:text-xl">8.62 M</div>
                </div>
              </div>
              <Chart2 />
            </div>
          </div>
        </div>
        <div className="pt-10 md:pt-[160px] box-px">
          <div className="between-center gap-[10px] md:gap-[140px] tec-flex1">
            <div className="w-full md:w-[40%] flex-shrink-0">
              <div className="pb-[10px] md:pb-[25px] text-[14px] md:text-[32px] font-semibold">
                <span className="text-white">{t('t245')}</span>
              </div>
              <div className="pb-[15px] md:pb-[40px] text-[12px] md:text-[18px] font-medium">
                <span className="text-green81">
                  {t('t246')}
                </span>
              </div>
              <div className="pt-[8px] md:pt-5 pb-[15px] md:pb-[30px] text-[12px] md:text-xl text-grayb2">
                {t('t247')}
              </div>
              <div className="pt-[8px] md:pt-5 pb-[15px] md:pb-[30px] text-[12px] md:text-xl text-grayb2">
                {t('t248')}
              </div>
            </div>
            <div className="flex-auto">
              <div className="between-center gap-[10px] md:gap-[30px] tec-add-1 mb-5 md:mb-[100px]">
                <div className="w-9 md:w-[56px] flex-shrink-0 tec-add-2 center-center">
                  <img src='/img/img111.png' className="w-[22px] md:w-full" alt="" />
                </div>
                <div className="tec-bb flex-auto">
                  <div className="text-[14px] md:text-[32px] text-white font-semibold">
                    {t('t249')}
                  </div>
                </div>
              </div>
              <div className="between-center gap-[10px] md:gap-[30px] tec-add-1 mb-5 md:mb-[100px]">
                <div className="w-9 md:w-[56px] flex-shrink-0 tec-add-2 center-center">
                  <img src='/img/img112.png' className="w-[22px] md:w-full" alt="" />
                </div>
                <div className="tec-bb flex-auto">
                  <div className="text-[14px] md:text-[32px] text-white font-semibold">
                    {t('t250')}
                  </div>
                </div>
              </div>
              <div className="between-center gap-[10px] md:gap-[30px] tec-add-1">
                <div className="w-9 md:w-[56px] flex-shrink-0 tec-add-2 center-center">
                  <img src='/img/img113.png' className="w-[22px] md:w-full" alt="" />
                </div>
                <div className="tec-bb flex-auto">
                  <div className="text-[14px] md:text-[32px] text-white font-semibold">
                    {t('t251')}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className="box-px pt180">
          <div className="pb-[10px] md:pb-[115px] text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">
              {t('t252')}
            </div>
            <div className="text-white text-[12px] md:text-xl">
              {t('t253')}
            </div>
          </div>
          <div className='add-grid3'>
            <div className='column-center gap-[10px]'>
              <img src="/img/img120.png" className='w-8 md:w-[64px]' alt="" />
              <div className="text-[14px] md:text-2xl font-semibold text-white break">
                {t('t254')}
              </div>
              <div className="text-grayb2 text-[12px] md:text-base leading-[2] break">
                {t('t255')}
              </div>
            </div>
            <div className='column-center gap-[10px]'>
              <img src="/img/img121.png" className='w-8 md:w-[64px]' alt="" />
              <div className="text-[14px] md:text-xl font-semibold text-white break">
                {t('t256')}
              </div>
              <div className="text-grayb2 text-[12px] md:text-base leading-[2] break">
                {t('t257')}
              </div>
            </div>
            <div className='column-center gap-[10px]'>
              <img src="/img/img122.png" className='w-8 md:w-[64px]' alt="" />
              <div className="text-[14px] md:text-xl font-semibold text-white break">
                {t('t258')}
              </div>
              <div className="text-grayb2 text-[12px] md:text-base leading-[2] break">
                {t('t259')}
              </div>
            </div>
            <div className='column-center gap-[10px]'>
              <img src="/img/img123.png" className='w-8 md:w-[64px]' alt="" />
              <div className="text-[14px] md:text-xl font-semibold text-white break">
                {t('t260')}
              </div>
              <div className="text-grayb2 text-[12px] md:text-base leading-[2] break">
                {t('t261')}
              </div>
            </div>
            <div className='column-center gap-[10px]'>
              <img src="/img/img124.png" className='w-8 md:w-[64px]' alt="" />
              <div className="text-[14px] md:text-xl font-semibold text-white break">
                {t('t262')}
              </div>
              <div className="text-grayb2 text-[12px] md:text-base leading-[2] break">
                {t('t263')}
              </div>
            </div>
            <div className='column-center gap-[10px]'>
              <img src="/img/img125.png" className='w-8 md:w-[64px]' alt="" />
              <div className="text-[14px] md:text-xl font-semibold text-white break">
                {t('t264')}
              </div>
              <div className="text-grayb2 text-[12px] md:text-base leading-[2] break">
                {t('t265')}
              </div>
            </div>
          </div>
        </div>
        <div className="box-px pt180">
          <div className="pb-[15px] md:pb-[60px] text-center">
            <div className="text-[18px] md:text-[48px] text-white font-semibold">
              {t('t266')}
            </div>
          </div>
          <div>
            <img src='/img/img114.png' className="w-full tobig" alt="" />
          </div>
        </div>
        <div className='pt-10 md:pt-[180px] box-px'>
          <div className="pb-[10px] md:pb-[115px] text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">
              {t('t267')}
            </div>
            <div className="text-white text-[12px] md:text-xl">
              {t('t268')}
            </div>
          </div>
          <div className='tec-grid1'>
            <div className='tecg1-item between-center gap-[10px] md:gap-[30px]'>
              <div className='tecg1-item-img center-center' style={{
                backgroundImage: `url('/img/img116.png')`
              }}>
              </div>
              <div className="flex-auto">
                <div className="text-[14px] md:text-2xl font-semibold text-white mb-[10px] md:mb-5">
                  {t('t269')}
                </div>
                <div className="text-grayb2 text-[12px] md:text-base leading-[2]">
                  {t('t270')}
                </div>
              </div>
            </div>
            <div className='tecg1-item between-center gap-[10px] md:gap-[30px]'>
              <div className='tecg1-item-img center-center' style={{
                backgroundImage: `url('/img/img117.png')`
              }}>
              </div>
              <div className="flex-auto">
                <div className="text-[14px] md:text-2xl font-semibold text-white mb-[10px] md:mb-5">
                  {t('t271')}
                </div>
                <div className="text-grayb2 text-[12px] md:text-base leading-[2]">
                  {t('t272')}
                </div>
              </div>
            </div>
            <div className='tecg1-item between-center gap-[10px] md:gap-[30px]'>
              <div className='tecg1-item-img center-center' style={{
                backgroundImage: `url('/img/img118.png')`
              }}>
              </div>
              <div className="flex-auto">
                <div className="text-[14px] md:text-2xl font-semibold text-white mb-[10px] md:mb-5">
                  {t('t273')}
                </div>
                <div className="text-grayb2 text-[12px] md:text-base leading-[2]">
                  {t('t274')}
                </div>
              </div>
            </div>
            <div className='tecg1-item between-center gap-[10px] md:gap-[30px]'>
              <div className='tecg1-item-img center-center' style={{
                backgroundImage: `url('/img/img119.png')`
              }}>
              </div>
              <div className="flex-auto">
                <div className="text-[14px] md:text-2xl font-semibold text-white mb-[10px] md:mb-5">
                  {t('t275')}
                </div>
                <div className="text-grayb2 text-[12px] md:text-base leading-[2]">
                  {t('t276')}
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className="box-px pt180">
          <div className="pb-[10px] md:pb-[115px] text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">
              {t('t277')}
            </div>
            <div className="text-white text-[12px] md:text-xl">
              {t('t278')}
            </div>
          </div>
          <div>
            <img src='/img/img115.png' className="w-full tobig" alt="" />
          </div>
        </div>
        <div className='h-[40px] md:h-[100px]'></div>
      </div>
    </TranslationsProvider>
  )
}
