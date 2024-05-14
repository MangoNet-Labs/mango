'use client';
import { useTranslation } from 'react-i18next';
import '@/styles/homeBox2.scss'
import { selectConfig } from '@/lib/storemodules/appSlice'
import { useSelector } from "react-redux";
import { ConfigType } from '@/interface'

export default function homeBox2({ config }: { config: ConfigType }) {
  const { t } = useTranslation();
  // const config = useSelector(selectConfig)

  return (
    <div className='home-box2-style'>
      <div className="box2 box-px">
        <div className="b2item">
          <div className="text-center">
            <div className="text-white text-[16px] md:text-[48px] font-bold mb-5">{config.Finality}</div>
            <div className="text-[12px] md:text-2xl text-grayb2">{t('t144')}</div>
          </div>
          <div className="text-center">
            <div className="text-white text-[16px] md:text-[48px] font-bold mb-5">{config.Second}</div>
            <div className="text-[12px] md:text-2xl text-grayb2">{t('t145')}</div>
          </div>
          <div className="text-center">
            <div className="text-white text-[16px] md:text-[48px] font-bold mb-5">{config.Transactions}</div>
            <div className="text-[12px] md:text-2xl text-grayb2">{t('t146')}</div>
          </div>
          <div className="text-center">
            <div className="text-white text-[16px] md:text-[48px] font-bold mb-5">{config.Users}</div>
            <div className="text-[12px] md:text-2xl text-grayb2">{t('t147')}</div>
          </div>
        </div>
      </div>
    </div>
  )
}
