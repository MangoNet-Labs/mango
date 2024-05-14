"use client"
import { useTranslation } from 'react-i18next';
import type { CollapseProps } from 'antd';
import { message, Collapse } from 'antd';
import { useRouter } from 'next/navigation'
import '@/styles/footers.scss'
import { ConfigType } from '@/interface'

export default function index({ config }: { config: ConfigType }) {
  const router = useRouter()
  const { t, i18n } = useTranslation();
  // const config = useSelector(selectConfig)
  const [messageApi, contextHolder] = message.useMessage();

  const notOpen = () => {
    messageApi.open({
      type: 'warning',
      content: t('t2')
    });
  }

  const getUrl = () => {
    const lang = i18n.language
    return `${process.env.NEXT_PUBLIC_INVITE_DOMAIN}/pdf/${lang == 'kr' ? 'kr/MangoWhitepaperKr' : lang == 'zhTw' ? 'zhTw/MangoNetwork_whitepaper_v1.2_Chinese' : 'en/MangoNetwork_whitepaper_v1.2'}.pdf`
  }

  const windowOpen = (url: string) => {
    if (url) {
      window.open(url)
    }
  }

  const goPage = (url: string) => {
    router.push(url)
  }

  const items: CollapseProps['items'] = [
    {
      key: '1',
      label: <span className="text-sm md:text-lg text-gray83 font-semibold">{t('t4')}</span>,
      children: <div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => goPage('/technology')}>{t('t5')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => goPage('/chain')}>{t('t6')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => goPage('/foundation')}>{t('t7')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => goPage('/blogList')}>{t('t8')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => goPage('/brand')}>{t('t9')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={notOpen}>{t('t10')}</span>
        </div>
      </div>
    },
    {
      key: '2',
      label: <span className="text-sm md:text-lg text-gray83 font-semibold">{t('t11')}</span>,
      children: <div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen('https://docs.mangonet.io/')}>{t('t12')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen('https://github.com/MangoNet-Labs/mango')}>Github</span>
        </div>
      </div>
    },
    {
      key: '3',
      label: <span className="text-sm md:text-lg text-gray83 font-semibold">{t('t13')}</span>,
      children: <div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => goPage('/activity')}>{t('t14')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => goPage('/community')}>{t('t15')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen(config.discord)}>Discord</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen(config.tme)}>Telegram</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen(config.medium)}>Medium</span>
        </div>
      </div>
    },
    {
      key: '4',
      label: <span className="text-sm md:text-lg text-gray83 font-semibold">{t('t16')}</span>,
      children: <div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => goPage('/dapp')}>{t('t17')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen('https://mgoscan.com/')}>{t('t18')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => notOpen}>{t('t19')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => notOpen}>{t('t20')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen('https://docs.mangonet.io/nodes/')}>{t('t21')}</span>
        </div>
      </div>
    },
    {
      key: '5',
      label: <span className="text-sm md:text-lg text-gray83 font-semibold">{t('t22')}</span>,
      children: <div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen(getUrl())}>{t('t23')}</span>
        </div>
        <div className="text-xs text-white mb-[10px]">
          <span className="cursor-pointer" onClick={() => windowOpen('https://mgoscan.com/')}>{t('t24')}</span>
        </div>
      </div>
    },
  ]

  return (
    <>
      {contextHolder}
      <div className='footer-style box-px'>
        <div className='gap-[70px] between-start pb-[120px]'>
          <div>
            <img src="/img/logo.png" className="w-[88px] mb-6" alt="" />
            <div className="w-[60px] bg-green81 h-[1px] mb-5"></div>
            <div className="text-sm text-gray83 mb-5">{t('t3')}</div>
            <div className="start-center gap-7">
              <img src="/img/img38.png" className="w-5 be_pointer" onClick={() => windowOpen(config.discord)} alt="" />
              <img src="/img/img106.png" className="w-5 be_pointer" onClick={() => windowOpen(config.medium)} alt="" />
              <img src="/img/img40.png" className="w-5 be_pointer" onClick={() => windowOpen(config.github)} alt="" />
              <img src="/img/img41.png" className="w-5 be_pointer" onClick={() => windowOpen(config.tme)} alt="" />
              <img src="/img/img42.png" className="w-5 be_pointer" onClick={() => windowOpen(config.youtube)} alt="" />
              <img src="/img/img43.png" className="w-5 be_pointer" onClick={() => windowOpen(config.twitter)} alt="" />
            </div>
            <div className="between-center"></div>
          </div>
          <div className="flex-auto ft-right-grid">
            <div className="fr-item">
              <span className="fr-item-title">{t('t4')}</span>
              <span onClick={() => goPage('/technology')}>{t('t5')}</span>
              <span onClick={() => goPage('/chain')}>{t('t6')}</span>
              <span onClick={() => goPage('/foundation')}>{t('t7')}</span>
              <span onClick={() => goPage('/blogList')}>{t('t8')}</span>
              <span onClick={() => goPage('/brand')}>{t('t9')}</span>
              <span onClick={notOpen}>{t('t10')}</span>
            </div>
            <div className="fr-item">
              <span className="fr-item-title">{t('t11')}</span>
              <span onClick={() => windowOpen('https://docs.mangonet.io/')}>{t('t12')}</span>
              <span onClick={() => windowOpen('https://github.com/MangoNet-Labs/mango')}>Github</span>
            </div>
            <div className="fr-item">
              <span className="fr-item-title">{t('t13')}</span>
              <span onClick={() => goPage('/activity')}>{t('t14')}</span>
              <span onClick={() => goPage('/community')}>{t('t15')}</span>
              <span onClick={() => windowOpen(config.discord)}>Discord</span>
              <span onClick={() => windowOpen(config.tme)}>Telegram</span>
              <span onClick={() => windowOpen(config.medium)}>Medium</span>
            </div>
            <div className="fr-item">
              <span className="fr-item-title">{t('t16')}</span>
              <span onClick={() => goPage('/dapp')}>{t('t17')}</span>
              <span onClick={() => windowOpen('https://mgoscan.com/')}>{t('t18')}</span>
              <span onClick={notOpen}>{t('t19')}</span>
              <span onClick={notOpen}>{t('t20')}</span>
              <span onClick={() => windowOpen('https://docs.mangonet.io/nodes/')}>{t('t21')}</span>
            </div>
            <div className="fr-item">
              <span className="fr-item-title">{t('t22')}</span>
              <span onClick={() => windowOpen(getUrl())}>{t('t23')}</span>
              <span onClick={() => windowOpen('https://mgoscan.com/')}>{t('t24')}</span>
            </div>
          </div>
        </div>
        <div className="text-sm text-gray83 text-center bb32">Copyright © 2024 MangoNet Labs.All Rights Reserved</div>
        <div className="text-sm text-gray83 text-center">
          <span className="be_hover_underline" onClick={() => windowOpen(config.service)}>
            {t('t25')}
          </span>｜<span className="be_hover_underline" onClick={() => windowOpen(config.privacy)}>
            {t('t26')}
          </span>
        </div>
      </div>
      <div className='footer-h5-style'>
        <div className="px-6 pt-6 pb-4 fh-bb">
          <img src="/img/logo.png" className="w-[42px] mb-[10px]" alt="" />
          <div className="text-xs text-gray83 mb-[5px]">{t('t3')}</div>
          <div className="w-[30px] bg-green81 h-[1px] mb-[10px]"></div>
          <div className="start-center gap-6">
            <img src="/img/img38.png" className="w-4 be_pointer" onClick={() => windowOpen(config.discord)} alt="" />
            <img src="/img/img106.png" className="w-4 be_pointer" onClick={() => windowOpen(config.medium)} alt="" />
            <img src="/img/img40.png" className="w-4 be_pointer" onClick={() => windowOpen(config.github)} alt="" />
            <img src="/img/img41.png" className="w-4 be_pointer" onClick={() => windowOpen(config.tme)} alt="" />
            <img src="/img/img42.png" className="w-4 be_pointer" onClick={() => windowOpen(config.youtube)} alt="" />
            <img src="/img/img43.png" className="w-4 be_pointer" onClick={() => windowOpen(config.twitter)} alt="" />
          </div>
        </div>
        <div className='py-[15px] px-6 fh-bb'>
          <Collapse
            expandIconPosition='end'
            items={items}
            ghost
            accordion
          />
        </div>
        <div className="pt-4 md:pt-0 text-xs md:text-sm text-gray83 text-center bb32">Copyright © 2024 MangoNet Labs.All Rights Reserved</div>
        <div className="pb-3 md:pb-0 text-xs md:text-sm text-gray83 text-center">
          <span className="be_hover_underline" onClick={() => windowOpen(config.service)}>{t('t25')}</span>｜
          <span className="be_hover_underline" onClick={() => windowOpen(config.privacy)}>{t('t26')}</span>
        </div>
      </div>
    </>
  )
}
