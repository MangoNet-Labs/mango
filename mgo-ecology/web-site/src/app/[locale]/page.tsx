import '@/styles/homeStyle.scss'
import initTranslations from '@/app/i18n'
import TranslationsProvider from '@/components/TranslationsProvider';
import { ParamsLocale, IndexInfoType } from '@/interface'
import clsx from 'clsx'
import { Chiq_Reduced_Bold } from '@/font/font'
import { getLangType } from '@/utils/index'
import { $isrGet, $ssrGet } from '@/utils/request'
import Image from 'next/image'
import type { CollapseProps } from 'antd';
import { Progress, Collapse } from 'antd';
import HomeBox5 from './_components/homeBox5'
import HomeBox2 from './_components/homeBox2'
import HomeBox1Btns from './_components/homeBox1Btns'
import Chart1 from './_components/chart1'
import Chart2 from './_components/chart2'

const i18nNamespaces = ['common'];

export default async function Home({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);
  const [config, res] = await Promise.all([
    (async () => {
      const result = await $ssrGet('/configList')
      return result
    })(),
    (async () => {
      const result = await $isrGet(`/indexContent?language=${getLangType(locale)}`)
      return result
    })()
  ])

  const OmniChain: IndexInfoType[] = res.data['0']
  const BasedOn: IndexInfoType[] = res.data['1']
  const MangoModular: IndexInfoType[] = res.data['2']
  const expandedNames: string = res.data['2'][0].ID.toString()
  const Getinvolved: IndexInfoType[] = res.data['3']

  const items: CollapseProps['items'] = MangoModular.map(item => ({
    key: item.ID.toString(),
    label: <span className='text-sm md:text-lg text-white font-semibold'>{item.title}</span>,
    children: <span className='text-[12px] md:text-base text-grayb2 pb-4'>{item.introduction}</span>,
  }))


  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className="home-style">
        <div className='box1 text-white'>
          <div className={
            clsx('b1text text-center mb-[5px] md:mb-[10px]', Chiq_Reduced_Bold.className)
          }>Mango Network</div>
          <div className={
            clsx('text-[18px] md:text-[28px] text-center text-gray83 md:text-white mb-[5px] md:mb-0', Chiq_Reduced_Bold.className)
          }>{t('t140')}</div>
          <div className={
            clsx('text-[14px] md:text-[18px] text-center mb-9', Chiq_Reduced_Bold.className)
          }>{t('t141')}</div>
          <HomeBox1Btns />
          <div className='b1iconlist'>
            <img src="/img/bg2.png" className="w-full" alt="" />
            <img src="/img/img10.png" className="absolute top-[45.55%] left-[-2.9752%] w-[5.95%] be_scaletogig" alt="" />
            <img src="/img/img11.png" className="absolute top-[42.0749%] left-[17.1%] w-[4.2975%] be_scaletogig" alt="" />
            <img src="/img/img12.png" className="absolute top-[15.562%] left-[33.72%] w-[5.62%] be_scaletogig" alt="" />
            <img src="/img/img13.png" className="absolute top-[19.31%] left-[60.3305%] w-[5.62%] be_scaletogig" alt="" />
            <img src="/img/img14.png" className="absolute top-[42.075%] left-[78.347%] w-[4.2975%] be_scaletogig" alt="" />
            <img src="/img/img16.png" className="absolute top-[48.415%] right-[-2.9752%] w-[5.95%] be_scaletogig" alt="" />
            <img src="/img/img15.png" className="absolute bottom-[-9.8%] left-[12.9752%] w-[5.95%] add-animate z-[2]" alt="" />
            <img src="/img/img17.png" className="absolute bottom-[-20.75%] left-[9.34%] w-[13.225%]" alt="" />
            <img src="/img/img18.png" className="absolute bottom-[-2.0173%] left-[44.96%] w-[9.917%] add-animate z-[2]" alt="" />
            <img src="/img/img19.png" className="absolute bottom-[-19.885%] left-[43.3%] w-[13.225%]" alt="" />
            <img src="/img/img20.png" className="absolute bottom-[-9.51%] right-[12.066%] w-[5.95%] add-animate z-[2]" alt="" />
            <img src="/img/img21.png" className="absolute bottom-[-20.75%] right-[8.43%] w-[13.225%]" alt="" />
            <div className="absolute left-[5.785%] top-[-6.63%] pos-kong"></div>
            <div className="absolute left-[48.18%] top-[2.0173%] pos-kong"></div>
            <div className="absolute right-[5.29%] top-[-6.63%] pos-kong"></div>
          </div>
        </div>
        <HomeBox2 config={config.data} />
        <div className="box3 box-px">
          <div className="text-center text-white">
            <div className="text-[18px] md:text-[48px] font-semibold mb-2">{t('t148')}</div>
            <div className="text-[12px] md:text-xl">{t('t149')}</div>
          </div>
          <div className="b3-grid">
            {
              OmniChain.map(item => (
                <div className="text-left md:text-center b3-bg1" key={item.ID}>
                  <img src={item.image} className="w-7 md:w-[56px] mb-[10px] md:mb-[30px]" alt="" />
                  <div className="text-sm md:text-lg font-bold mb-[5px] md:mb-[10px] text-white">{item.title}</div>
                  <div className="text-grayb2 text-xs md:text-sm" dangerouslySetInnerHTML={{ __html: item.introduction }}></div>
                </div>
              ))
            }
          </div>
        </div>
        <div className="box4 box-px">
          <div className="text-center text-white">
            <div className="text-[16px] md:text-[48px] font-semibold mb-2">{t('t150')}</div>
            <div className="text-[12px] md:text-xl">{t('t151')}</div>
          </div>
          <div className="pt-5 md:pt-[70px]">
            <img src="/img/img109.png" className="w-full tobig" alt="" />
          </div>
        </div>
        <div className="box4 box-px">
          <div className="text-center text-white">
            <div className="text-[16px] md:text-[48px] font-semibold mb-2">{t('t152')}</div>
            <div className="text-[12px] md:text-xl">{t('t153')}</div>
          </div>
          <div className="pt-5 md:pt-[70px]">
            <img src="/img/img27.png" className="w-full tobig" alt="" />
          </div>
        </div>
        <div className="box4 box-px">
          <div className="between-center gap-[60px] b4-flex1">
            <div className="w-full md:w-[39.571%] flex-shrink-0 text-center">
              <img src="/img/img105.png" className="w-[70%] md:w-full" alt="" />
            </div>
            <div>
              <div className="text-[18px] md:text-[48px] text-center md:text-left font-semibold text-white mb-6">{t('t154')}</div>
              <div className="b4-listtopline">
                {
                  BasedOn.map(item => (
                    <div className="b4-list-item" key={item.ID}>
                      <div className="text-sm md:text-2xl text-white font-semibold">{item.title}</div>
                      <div className="text-xs md:text-base text-grayb2">{item.introduction}</div>
                    </div>
                  ))
                }
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
        {
          MangoModular.length ?
            <div className='box4 box-px'>
              <div className="between-center gap-[40px] b4-flex2">
                <div className="w-full md:w-[41.43%] flex-shrink-0 center-center">
                  <img src="/img/img29.png" className="w-[84%] md:w-full add-tobig" alt="" />
                </div>
                <div className="flex-auto">
                  <div className="text-[18px] md:text-[48px] font-semibold text-white mb-[25px] md:mb-[50px] text-center md:text-left">{t('t173')}</div>
                  <div className="b4-coll">
                    <Collapse
                      expandIconPosition='end'
                      items={items}
                      ghost
                      accordion
                      defaultActiveKey={expandedNames}
                    />
                  </div>
                </div>
              </div>
            </div> :
            <></>
        }
        <div className="box4 box-px">
          <div className="text-[18px] md:text-[48px] font-semibold mb-[15px] md:mb-[80px] text-white text-center md:text-left">{t('t174')}</div>
          <div className="b4-pic-grid">
            <div className="overflow-hidden">
              <img src="/img/img30.png" className="w-full tobig" alt="" />
            </div>
            <div className="overflow-hidden">
              <img src="/img/img31.png" className="w-full tobig" alt="" />
            </div>
          </div>
          <div className="b4-pic-grid2">
            <div className="overflow-hidden">
              <img src="/img/img32.jpg" className="w-full tobig" alt="" />
            </div>
            <div className="overflow-hidden">
              <img src="/img/img33.jpg" className="w-full tobig" alt="" />
            </div>
          </div>
        </div>
        <HomeBox5 Getinvolved={Getinvolved} />
        <div className="h-10 md:h-[124px]"></div>
      </div>
    </TranslationsProvider>
  );
}