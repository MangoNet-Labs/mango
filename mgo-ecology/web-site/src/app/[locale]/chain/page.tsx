import { AllChainType, ParamsLocale } from '@/interface';
import '@/styles/chain.scss'
import TranslationsProvider from '@/components/TranslationsProvider';
import { Chiq_Reduced_Bold } from '@/font/font'
import initTranslations from '@/app/i18n'
import clsx from 'clsx'
import ChainBox1Btn from './_components/chainBox1Btn'
import { $isrGet } from '@/utils/request'
import { getLangType } from '@/utils/index'
import type { CollapseProps } from 'antd';
import { Collapse } from 'antd';
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/chain', locale)
}

const i18nNamespaces = ['common'];

export default async function page({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);
  const res = await $isrGet(`/allChain?language=${getLangType(locale)}`)
  for (const key in res.data) {
    const valueList = res.data[key]
    for (let i = 0; i < valueList.length; i++) {
      const element = valueList[i];
      element.introduction = element.introductionJson ? JSON.parse(element.introductionJson) : ''
    }
  }

  const MangoApplicationArchitecture: AllChainType = res.data['5'][0]
  const MangoCrossChain: AllChainType = res.data['4'][0]
  res.data['0'][0].arr = Object.values(res.data['0'][0].introduction)
  const MultiChain: AllChainType = res.data['0'][0]
  const TechnicalArchitecture: AllChainType = res.data['3'][0]
  for (let i = 0; i < res.data['6'].length; i++) {
    const element = res.data['6'][i];
    element.arr = Object.values(element.introduction)
  }
  const TechnicalArchitectureAdvantages: AllChainType[] = res.data['6']
  const expandedNames: string = TechnicalArchitectureAdvantages[0].ID.toString()
  for (let i = 0; i < res.data['1'].length; i++) {
    const element = res.data['1'][i];
    element.arr = Object.values(element.introduction)
  }
  const allChain: AllChainType[] = res.data['1']
  const MultiChainApplication: AllChainType = res.data['2'][0]

  const items: CollapseProps['items'] = TechnicalArchitectureAdvantages.map(item => ({
    key: item.ID.toString(),
    label: <span className="text-sm md:text-lg text-white font-semibold">{item.title}</span>,
    children: <div className='text-[12px] md:text-base text-grayb2 pb-4'>
      {
        item.arr.map((it, i) => (
          <div key={i}>
            {i + 1}.{it}
          </div>
        ))
      }
    </div>
  }))

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className='chain-style'>
        <div className="box1">
          <div className={
            clsx('b1-text1', Chiq_Reduced_Bold.className)
          }>{t('t95')}</div>
          <div className="pt-[5px] md:pt-[10px] pb-[15px] md:pb-[110px] text-[12px] md:text-2xl text-white">
            {t('t96')}
          </div>
          <ChainBox1Btn />
        </div>
        <div className="box-px pt180">
          <div className="pb-[10px] md:pb-[115px] text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">{t('t99')}</div>
            <div className="text-white text-[12px] md:text-xl">{t('t100')}</div>
          </div>
          <div className="between-center gap-[15px] md:gap-[50px] cha-add-1">
            <div className="flex-auto">
              <div className="pb-[15px] md:pb-[50px] text-[14px] md:text-[32px] font-medium bb32">
                <span className="text-green81">Multi-chain</span>
                <span className="text-white">{MultiChain.title}</span>
              </div>
              {
                MultiChain?.arr.map((item, i) => (
                  <div className="pt-[15px] md:pt-5 pb-[15px] md:pb-[30px] bb32 text-[12px] md:text-xl text-grayb2" key={i}>
                    {i + 1}  {item}
                  </div>
                ))
              }
            </div>
            <div className="w-full md:w-[45%] flex-shrink-0 center-center">
              <div className="w-[70%] md:w-full addmovebg">
                <img src="/img/mbg.png" className="w-full" alt="" />
                <img src="/img/m.png" className="addmovebg-img" alt="" />
              </div>
            </div>
          </div>
        </div>
        <div className="box-px pt180">
          <div className="pb-[15px] md:pb-[80px] text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">{t('t101')}</div>
            <div className="text-white text-[12px] md:text-xl">{t('t102')}</div>
          </div>
          <div className="cha-grid1">
            {
              allChain.map(item => (
                <div className="chag1-item" key={item.ID}>
                  <img src={item.image} className="w-[42px] md:w-[84px] mb-[15px] md:mb-5" alt="" />
                  <div className="text-[14px] md:text-2xl text-white font-bold mb-[10px] md:mb-[30px]">{item.title}</div>
                  <div className="text-grayb2 text-[12px] md:text-lg">
                    {
                      item.arr.map((it, i) => (
                        <div className="mb-2" key={i}>
                          {i + 1}.{it}
                        </div>
                      ))
                    }
                  </div>
                </div>
              ))
            }
          </div>
        </div>
        <div className='box-px pt180'>
          <div className="pb-[15px] md:pb-[80px] text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">{MultiChainApplication?.title}</div>
            <div className="text-white text-[12px] md:text-xl">{MultiChainApplication?.introduction['1']}</div>
          </div>
          <div className='overflow-x-auto'>
            <table className='text-white w-full table-bg'>
              <tbody className='text-xs md:text-sm'>
                <tr className='text-sm md:text-2xl font-medium'>
                  <td>{t('t103')}</td>
                  <td>Multi-Chain</td>
                  <td className='whitespace-nowrap'>Omni-Chain</td>
                </tr>
                <tr>
                  <td className='text-sm md:text-2xl font-medium'>{t('t104')}</td>
                  <td className='whitespace-nowrap'>{t('t105')}</td>
                  <td className='whitespace-nowrap'>{t('t106')}</td>
                </tr>
                <tr>
                  <td className="text-sm md:text-2xl font-medium whitespace-nowrap">{t('t108')}</td>
                  <td>{t('t109')}</td>
                  <td>{t('t110')}</td>
                </tr>
                <tr>
                  <td className="text-sm md:text-2xl font-medium">{t('t111')}</td>
                  <td>{t('t112')}</td>
                  <td>{t('t113')}</td>
                </tr>
              </tbody>
            </table>
          </div>
          <div className="pt-[5px] md:pt-5 text-[12px] md:text-lg text-gray66 text-center">
            {t('t114')}
          </div>
        </div>
        <div className="box-px pt180">
          <div className="pb-[60px] text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">{TechnicalArchitecture?.title}</div>
            <div className="text-grayb2 text-[12px] md:text-xl">{TechnicalArchitecture?.introduction['1']}</div>
          </div>
          <div>
            <img src={TechnicalArchitecture?.image} className="w-full tobig" alt="" />
          </div>
        </div>
        <div className="box-px pt180">
          <div className="pb-[15px] md:pb-[60px] text-center">
            <div className="text-[18px] md:text-[48px] text-white font-semibold">{MangoCrossChain?.title}</div>
          </div>
          <div>
            <img src={MangoCrossChain?.image} className="w-full tobig" alt="" />
          </div>
        </div>
        <div className="box-px pt180">
          <div className="pb-[60px] text-center">
            <div className="text-[18px] md:text-[48px] text-white mb-[5px] md:mb-5 font-semibold">{MangoApplicationArchitecture?.title}</div>
            <div className="text-grayb2 text-[12px] md:text-xl">{MangoApplicationArchitecture?.introduction['1']}</div>
          </div>
          <div>
            <img src={MangoApplicationArchitecture?.image} className="w-full tobig" alt="" />
          </div>
        </div>
        <div className='box-px pt180 pb-[40px] md:pb-[180px]'>
          <div className='between-start gap-[25px] md:gap-[50px] cha-add-2'>
            <div className="w-full md:w-[50%] flex-shrink-0">
              <div className="text-[18px] md:text-[48px] text-white font-semibold pb-[10px] md:pb-[30px]">{t('t115')} </div>
              <div className="start-center cha-add-3">
                <div className="w-[40px] md:w-[80px] h-[2px] md:h-1 bg-green81"></div>
              </div>
            </div>
            <div className='b4-coll flex-auto'>
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
      </div>
    </TranslationsProvider>
  )
}
