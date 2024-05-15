"use client"
import { RightOutlined } from '@ant-design/icons'
import { useTranslation } from 'react-i18next';
import '@/styles/homeBox1Btns.scss'
import { useRouter } from 'next/navigation'

export default function homeBox1Btns() {
  const { t } = useTranslation();
  const router = useRouter()

  const windowOpen = (url: string) => {
    if (url) {
      window.open(url)
    }
  }

  return (
    <div className="center-center gap-5 mb-10 home-box1-btns-style">
      <button className="b1-btn be_pointer" onClick={() => windowOpen('https://docs.mangonet.io/')}>
        <span>{t('t142')}</span>
      </button>
      <button className="b1-btn gap-[10px] center-center be_pointer" onClick={() => router.push('/technology')}>
        <span>{t('t143')}</span>
        <RightOutlined style={{
          color: '#fff'
        }} />
      </button>
    </div>
  )
}
