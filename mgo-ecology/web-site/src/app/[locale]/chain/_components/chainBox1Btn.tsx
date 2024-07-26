"use client"
import { useTranslation } from 'react-i18next';
import { useRouter } from 'next/navigation'
import '@/styles/chainBox1Btn.scss'

export default function chainBox1Btn() {
  const { t } = useTranslation();
  const router = useRouter()

  return (
    <div className="chain-box1-btn-style">
      <div className="b1btn-box" onClick={() => router.push('/technology')}>
        <div className="b1-btn">
          <div className="center-center gap-[10px]">
            <span className="text-[12px] md:text-2xl text-black06">{t('t97')}</span>
            <img src="/img/img57.png" className="w-3 md:w-6" alt="" />
          </div>
        </div>
        <div className="b1-btn-pos">
          <div className="center-center gap-[10px]">
            <span className="text-[12px] md:text-2xl text-black06">{t('t97')}</span>
            <img src="/img/img57.png" className="w-3 md:w-6" alt="" />
          </div>
        </div>
        <div className="b1-btn-pos2">{t('t97')}</div>
      </div>
    </div>
  )
}
