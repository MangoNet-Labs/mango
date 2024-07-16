"use client"
import { $clientGet } from '@/utils/request'
import { useTranslation } from 'react-i18next';
import { useState, useEffect } from 'react'
import { getLangType, getLastPage } from '@/utils'
import { BlogType } from '@/interface';
import dayjs from 'dayjs'
import { useRouter } from 'next/navigation'
import { Pagination } from 'antd';

export default function listTitle() {
  const { t, i18n } = useTranslation();
  const router = useRouter()

  const [page, setPage] = useState(1)
  const [lastPage, setLastPage] = useState(1)
  const [blogList, setBlogList] = useState<BlogType[]>([])
  const getBlog = async (newPage?: number) => {
    const res = await $clientGet(`/blogList?language=${getLangType(i18n.language)}&type=1&page=${newPage ?? page}&pageSize=9`)
    const { total, list } = res.data
    const last_page = getLastPage(total, 9)

    setLastPage(last_page)
    setBlogList(list)
  }
  useEffect(() => {
    getBlog()
  }, [])

  const changePage = (current: number) => {
    setPage(current)
    getBlog(current)
  }

  return (
    <div className="bloglist-style">
      <div className='pt-[25px] md:pt-[80px] pb-[30px] md:pb-[100px]'>
        <div className='b4-list'>
          {
            blogList.map(item => (
              <div onClick={() => router.push(`/blogDetail/${item.ID}`)} className='b4-item be_pointer' key={item.ID}>
                <div className='b4i-img'>
                  <img src={item.image} alt="" />
                </div>
                <div className="b4i-box">
                  <div className="b4i-text1 ellipsis">{item.title}</div>
                  <div className="b4i-text2 ellipsis2" dangerouslySetInnerHTML={{ __html: item.introduction }}></div>
                </div>
              </div>
            ))
          }
        </div>
      </div>
      <div className='b2-page center-center '>
        <Pagination current={page} total={lastPage} pageSize={1} onChange={changePage} />
      </div>
    </div>
  )
}
