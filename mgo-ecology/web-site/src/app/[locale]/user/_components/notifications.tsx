import '@/styles/personal.scss'
import { Tabs, Pagination } from 'antd';
import { useTranslation } from 'react-i18next';
import { NotificationsType, ResponsesType, LikesReceivedType } from '@/interface';
import { useState, useEffect } from 'react'
import { getLangType, getLastPage } from '@/utils'
import { $clientGet } from '@/utils/request'
import { selectUserInfo } from '@/lib/storemodules/appSlice'
import { useSelector } from "react-redux";
import { useRouter } from 'next/navigation'
import dayjs from 'dayjs'


export default function activity() {
  const { t } = useTranslation();

  const items = [
    {
      label: <span className='text-xs md:text-lg font-semibold'>{t('t184')}</span>,
      key: '1',
      children: <Tab1 />,
      icon: <img src="/img/img98.png" alt="" className='w-3 md:w-4' />
    },
    {
      label: <span className='text-xs md:text-lg font-semibold'>{t('t183')}</span>,
      key: '2',
      children: <Tab2 />,
      icon: <img src="/img/img99.png" alt="" className='w-3 md:w-4' />
    },
    {
      label: <span className='text-xs md:text-lg font-semibold'>{t('t185')}</span>,
      key: '3',
      children: <Tab3 />,
      icon: <img src="/img/img100.png" alt="" className='w-3 md:w-4' />
    },
  ]

  return (
    <div className='pic-style'>
      <Tabs
        defaultActiveKey="1"
        centered
        items={items}
      />
    </div>
  )
}

function Tab1() {
  const router = useRouter()
  const userInfo = useSelector(selectUserInfo)
  const { t, i18n } = useTranslation();
  const [activityList, setActivityList] = useState<NotificationsType[]>([])
  const [page, setPage] = useState(1)
  const [lastPage, setLastPage] = useState(1)

  const changePage = (current: number) => {
    setPage(current)
    getActivity(current)
  }

  const getActivity = async (newPage?: number) => {
    const res = await $clientGet(`/notifications?language=${getLangType(i18n.language)}&page=${newPage ?? page}&pageSize=10&user_id=${userInfo.ID}`)
    const { list, total, pageSize } = res.data
    setActivityList(list)
    const last_page = getLastPage(total, pageSize)
    setLastPage(last_page)
  }

  useEffect(() => {
    getActivity()
  }, [])

  return (
    <div>
      {
        !activityList.length ?
          <div className="h-[60px] center-center">
            <span className="text-gray83 text-sm">{t('t131')}</span>
          </div> :
          <div className='pt-4 pb-5'>
            {
              activityList.map((item, i) => (
                <div className='py-[10px] px-5 between-center gap-[10px] bb1' key={i}>
                  <div className='py-[10px] px-5 between-center gap-[10px] bb1' key={i}>
                    <img src={item.notifications == 1 ? '/img/img85.png' : '/img/img86.png'} className="w-4" alt="" />
                    <div className="flex-auto text-xs md:text-sm">
                      <span className="text-white mr-[6px]">{item.userName}</span>
                      <span className="text-green81 be_hover_underline"
                        onClick={() => router.push(`/communityDetail/${item.forumId}/${item.forumcategoryId}`)}>
                        {item.title}</span>
                    </div>
                    <div className='text-xs md:text-sm text-gray83'>{dayjs(item.CreatedAt).format('MM/DD')}</div>
                  </div>
                </div>
              ))
            }
          </div>
      }
      <div className='pt-5 md:pt-10 pb-10 center-center'>
        <Pagination current={page} total={lastPage} pageSize={1} onChange={changePage} />
      </div>
    </div>
  )
}

function Tab2() {
  const router = useRouter()
  const userInfo = useSelector(selectUserInfo)
  const { t, i18n } = useTranslation();
  const [repliesList, setRepliesList] = useState<ResponsesType[]>([])
  const [page, setPage] = useState(1)
  const [lastPage, setLastPage] = useState(1)

  const changePage = (current: number) => {
    setPage(current)
    getReplies(current)
  }

  const getReplies = async (newPage?: number) => {
    const res = await $clientGet(`/responses?language=${getLangType(i18n.language)}&page=${newPage ?? page}&pageSize=10&user_id=${userInfo.ID}`)
    const { list, total, pageSize } = res.data
    setRepliesList(list)
    const last_page = getLastPage(total, pageSize)
    setLastPage(last_page)
  }

  useEffect(() => {
    getReplies()
  }, [])

  return (
    <div>
      {
        !repliesList.length ?
          <div className="h-[60px] center-center">
            <span className="text-gray83 text-sm">{t('t131')}</span>
          </div> :
          <div className='pt-4 pb-5'>
            {
              repliesList.map((item, i) => (
                <div className='py-[10px] px-5 between-center gap-[10px] bb1' key={i}>
                  <div className='py-[10px] px-5 between-center gap-[10px] bb1' key={i}>
                    <img src='/img/img86.png' className="w-4" alt="" />
                    <div className="flex-auto text-xs md:text-sm">
                      <span className="text-white mr-[6px]">{item.userName}</span>
                      <span className="text-green81 be_hover_underline"
                        onClick={() => router.push(`/communityDetail/${item.forumId}/${item.forumcategoryId}`)}>
                        {item.title}</span>
                    </div>
                    <div className='text-xs md:text-sm text-gray83'>{dayjs(item.CreatedAt).format('MM/DD')}</div>
                  </div>
                </div>
              ))
            }
          </div>
      }
      <div className='pt-5 md:pt-10 pb-10 center-center'>
        <Pagination current={page} total={lastPage} pageSize={1} onChange={changePage} />
      </div>
    </div>
  )
}

function Tab3() {
  const router = useRouter()
  const userInfo = useSelector(selectUserInfo)
  const { t, i18n } = useTranslation();
  const [likeList, setLikeList] = useState<LikesReceivedType[]>([])
  const [page, setPage] = useState(1)
  const [lastPage, setLastPage] = useState(1)

  const changePage = (current: number) => {
    setPage(current)
    getReplies(current)
  }

  const getReplies = async (newPage?: number) => {
    const res = await $clientGet(`/likesReceived?language=${getLangType(i18n.language)}&page=${newPage ?? page}&pageSize=10&user_id=${userInfo.ID}`)
    const { list, total, pageSize } = res.data
    setLikeList(list)
    const last_page = getLastPage(total, pageSize)
    setLastPage(last_page)
  }

  useEffect(() => {
    getReplies()
  }, [])

  return (
    <div>
      {
        !likeList.length ?
          <div className="h-[60px] center-center">
            <span className="text-gray83 text-sm">{t('t131')}</span>
          </div> :
          <div className='pt-4 pb-5'>
            {
              likeList.map((item, i) => (
                <div className='py-[10px] px-5 between-center gap-[10px] bb1' key={i}>
                  <img src="/img/img101.png" className="w-4" alt="" />
                  <div className="flex-auto text-xs md:text-sm">
                    <span className="text-white mr-[6px]">{item.userName}</span>
                    <span className="text-green81 be_hover_underline"
                      onClick={() => router.push(`/communityDetail/${item.forumId}/${item.forumcategoryId}`)}>
                      {item.title}</span>
                  </div>
                  <div className='text-xs md:text-sm text-gray83'>{dayjs(item.CreatedAt).format('MM/DD')}</div>
                </div>
              ))
            }
          </div>
      }
      <div className='pt-5 md:pt-10 pb-10 center-center'>
        <Pagination current={page} total={lastPage} pageSize={1} onChange={changePage} />
      </div>
    </div>
  )
}
