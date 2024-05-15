"use client"
import { useRouter } from 'next/navigation'
import { useTranslation } from 'react-i18next';
import '@/styles/user.scss'
import { selectUserInfo } from '@/lib/storemodules/appSlice'
import { useSelector } from "react-redux";
import { Tabs } from 'antd';
import Personal from './personal'
import Activity from './activity'
import Notifications from './notifications'

export default function index() {
  const router = useRouter()
  const { t } = useTranslation();
  const userInfo = useSelector(selectUserInfo)

  const items = [
    {
      label: <span className='text-sm md:text-2xl font-semibold'>{t('t210')}</span>,
      key: '1',
      children: <Personal />,
      icon: <img src="/img/img94.png" alt="" className='w-3 md:w-6' />
    },
    {
      label: <span className='text-sm md:text-2xl font-semibold'>{t('t211')}</span>,
      key: '2',
      children: <Activity />,
      icon: <img src="/img/img96.png" alt="" className='w-3 md:w-6' />
    },
    {
      label: <span className='text-sm md:text-2xl font-semibold'>{t('t212')}</span>,
      key: '3',
      children: <Notifications />,
      icon: <img src="/img/img98.png" alt="" className='w-3 md:w-6' />
    }
  ]

  return (
    <div className='user-style box-px'>
      <div className='bl1 pt-[15px] md:pt-10 pl-[15px] md:pl-10'>
        <div className="start-center mb-[15px] md:mb-[50px]">
          <div className="start-center gap-[10px] be_pointer" onClick={() => router.back()}>
            <img src="/img/img91.png" className="w-4" alt="" />
            <span className="text-sm md:text-lg text-green81 be_hover_underline">{t('t209')}</span>
          </div>
        </div>
        <div className="start-center pb-[15px] md:pb-5 gap-[10px]">
          <img src={userInfo.headerImg} className="w-7 md:w-[56px]" alt="" />
          <div className="text-white">
            <div className="text-sm md:text-lg font-semibold leading-[2]">{userInfo.userName}</div>
          </div>
        </div>
        <div>
          <Tabs
            defaultActiveKey="1"
            centered
            items={items}
          />
        </div>
      </div>
    </div>
  )
}
