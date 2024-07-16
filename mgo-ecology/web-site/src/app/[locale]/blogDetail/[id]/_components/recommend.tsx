"use client"
import '@/styles/blogdetail.scss'
import { useRouter } from 'next/navigation'
import { BlogType } from '@/interface';
import { $clientGet } from '@/utils/request'
import { getLangType } from '@/utils'
import { useTranslation } from 'react-i18next';
import { useState, useEffect } from 'react'
import dayjs from 'dayjs'

export default function change() {
  const router = useRouter()
  const { t, i18n } = useTranslation();

  const [recommendBlog, setRecommendBlog] = useState<BlogType[]>([])
  const getRecommendBlog = async () => {
    const res = await $clientGet(`/blogList?language=${getLangType(i18n.language)}&type=1&page=1&pageSize=3`)
    setRecommendBlog(res.data.list)
  }
  useEffect(() => {
    getRecommendBlog()
  }, [])

  return (
    <div className='blogdetail-style'>
      <div className='box4'>
        <div className='b4-title'>{t('t93')}</div>
        <div className='b4-list'>
          {
            recommendBlog.map(item => (
              <div onClick={() => router.replace(`/blogDetail/${item.ID}`)} className='b4-item be_pointer' key={item.ID}>
                <div className='b4i-img'>
                  <img src={`${item.image}`} alt="" />
                </div>
                <div className="b4i-box">
                  <div className="b4i-text1 ellipsis2">{item.title}</div>
                  <div className="b4i-text2 ellipsis2" dangerouslySetInnerHTML={{ __html: item.introduction }}></div>
                  <div className="b4i-text3">{dayjs(item.releaseTime).format('YYYY-MM-DD')}</div>
                </div>
              </div>
            ))
          }
        </div>
      </div>
    </div>
  )
}
