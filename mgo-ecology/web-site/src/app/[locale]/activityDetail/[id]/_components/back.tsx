"use client"
import { useRouter } from 'next/navigation'
import { useTranslation } from 'react-i18next'
import { LeftOutlined } from '@ant-design/icons'

export default function back() {
  const router = useRouter()
  const { t } = useTranslation()

  return (
    <div className='acdetail-style'>
      <div className="start-center gap-[5px] md:gap-[10px] be_hover pr-1" onClick={() => router.back()}>
        {/* <van-icon name="arrow-left" color="#fff" size="18" /> */}
        <LeftOutlined style={{
          color: '#fff',
          fontSize: '16px'
        }} />
        <span className="text-[14px] md:text-lg text-white">{t('t14')}</span>
      </div>
    </div>
  )
}
