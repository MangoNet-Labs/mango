"use client"
import { $clientGet } from '@/utils/request'
import { useTranslation } from 'react-i18next';
import { useState, useEffect } from 'react'
import { getLangType } from '@/utils'
import { BlogType } from '@/interface';
import dayjs from 'dayjs'
import { useRouter } from 'next/navigation'

export default function listTitle() {
  const { t, i18n } = useTranslation();
  const router = useRouter()

  const [recommendBlog, setRecommendBlog] = useState<BlogType[]>([])
  const getRecommendBlog = async () => {
    const res = await $clientGet(`/blogList?language=${getLangType(i18n.language)}&type=0&page=1&pageSize=1`)
    setRecommendBlog(res.data.list)
  }
  useEffect(() => {
    getRecommendBlog()
  }, [])

  return (
    <div className="bloglist-style">
      <div className='box1 box-px'>
        {
          recommendBlog.map(item => (
            <div onClick={() => router.push(`/blogDetail/${item.ID}`)} className='be_pointer' key={item.ID}>
              <div className="b1-row1 between-center">
                <div className="b1r1-text1 ellipsis">{item.title}</div>
                <div className="b1r1-text2">{dayjs(item.releaseTime).format('YYYY-MM-DD')}</div>
              </div>
              <div className="b1-row2 ellipsis2">
                {item.introduction}
              </div>
            </div>
          ))
        }
      </div>
    </div>
  )
}
