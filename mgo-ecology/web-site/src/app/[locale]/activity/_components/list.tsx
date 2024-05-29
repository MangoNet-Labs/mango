"use client"
import { $clientGet } from '@/utils/request'
import '@/styles/activity.scss'
import { useTranslation } from 'react-i18next';
import { useState, useEffect } from 'react'
import { getLangType } from '@/utils'
import { EventListItemType } from '@/interface';
import { useRouter } from 'next/navigation'
import dayjs from 'dayjs'

export default function list() {
  const { t, i18n } = useTranslation();
  const router = useRouter()

  const [eventList, setEventList] = useState<EventListItemType[]>([])
  const getEventList = async () => {
    const res = await $clientGet(`/eventList?language=${getLangType(i18n.language)}`)
    setEventList(res.data)
  }
  useEffect(() => {
    getEventList()
  }, [])

  return (
    <div className='activity-style'>
      <div className='box-px pb-[55px] md:pb-[144px] btff02 bbff02 ac-grid1'>
        {
          eventList.map(item => (
            <div onClick={() => router.push(`/activityDetail/${item.ID}`)} className='btff02 bbff02 blff02 brff02 p-[15px] md:p-10 clear-left ac-item be_hover' key={item.ID}>
              <div className='w-full'>
                <div className='relative w-full'>
                  <img src={item.image} className="w-full h-auto" alt="" />
                </div>
                <div className="pt-[15px] md:pt-5 text-grayb2 text-[12px] md:text-lg">{dayjs(item.endTime).format('YYYY-MM-DD HH:mm')}</div>
                <div className="text-white text-[16px] md:text-[32px] pt-[5px] md:pt-[10px] ellipsis2">{item.title}</div>
                {/* <div className="text-[12px] md:text-xl text-grayb2 pt-[15px] md:pt-5 ellipsis2">{item.introduction}</div> */}
                <div className="text-[12px] md:text-xl text-grayb2 pt-[15px] md:pt-5 ellipsis2" dangerouslySetInnerHTML={{ __html: item.introduction }}></div>
              </div>
              <img src="/img/img37.png" className="w-[18px] md:w-6" alt="" />
            </div>
          ))
        }
      </div>
    </div>
  )
}
