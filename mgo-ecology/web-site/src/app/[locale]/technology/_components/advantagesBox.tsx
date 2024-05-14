"use client"
import { NetworkType } from '@/interface';
import { windowOpen } from '@/utils'
import { useTranslation } from 'react-i18next';
import clsx from 'clsx'


export default function advantagesBox({ Advantages }: { Advantages: NetworkType[] }) {
  const { t } = useTranslation();

  return (
    <div className="technology-style">
      <div className='pt-[33px] md:pt-[180px]'>
        <div className="text-[18px] md:text-[48px] text-white pb-[10px] md:pb-[80px] text-center font-semibold">
          {t('t180')}
        </div>
        <div className="add-box2 box-px">
          <div className="box2">
            {
              Advantages.map(item => (
                <div className={
                  clsx('bl32 br32 p-[15px] md:p-10 b2-item', {
                    'be_hover': item.url
                  })
                } key={item.ID}>
                  <div>
                    <div className="w-[30px] md:w-[80px] h-[2px] md:h-1 bg-green81 mb-[10px] md:mb-5"></div>
                    <div className="text-[14px] md:text-[28px] text-white mb-[5px] md:mb-5">{item.title}</div>
                    <div className="leading-[2] text-[12px] md:text-base text-grayce">
                      {item.introduction}
                    </div>
                  </div>
                </div>
              ))
            }
          </div>
        </div>
      </div>
    </div>
  )
}
