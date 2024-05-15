"use client"
import '@/styles/homeBox5.scss'
import clsx from 'clsx'
import { IndexInfoType } from '@/interface';
import Image from 'next/image'
import Link from 'next/link'

export default function homeBox5({ Getinvolved }: { Getinvolved: IndexInfoType[] }) {
  return (
    <div className='home-box5-style'>
      <div className="box5 box-px">
        {/* <div @click="goPage(item.url)" className="p-[15px] md:p-10 b5bl b5br b5bb" :className="item.url ? 'be_hover' : ''"
          v-for="item in Getinvolved" :key="item.id">
          <div>
            <img :src="imgurl + item.image" className="w-[28px] md:w-[56px] mb-[10px] md:mb-9" alt="">
            <div className="text-base md:text-[32px] text-white font-semibold mb-[5px] md:mb-5">{{ item.title }}</div>
            <div className="leading-[2] mb-[10px] md:mb-[55px] text-grayb2 text-sx md:text-xl">{{ item.introduction }}</div>
          </div>
          <img src="@/assets/img/img37.png" className="w-4 md:w-6" alt="">
        </div> */}
        {
          Getinvolved.map(item => (
            <Link key={item.ID} href={item.url} className={
              clsx('p-[15px] md:p-10 b5bl b5br b5bb', {
                'be_hover': item.url
              })
            }>
              <div>
                <img src={item.image} className="w-[28px] md:w-[56px] mb-[10px] md:mb-9" alt="" />
                <div className="text-base md:text-[32px] text-white font-semibold mb-[5px] md:mb-5">{item.title}</div>
                <div className="leading-[2] mb-[10px] md:mb-[55px] text-grayb2 text-sx md:text-xl" dangerouslySetInnerHTML={{ __html: item.introduction }}></div>
              </div>
              <Image src="/img/img37.png" alt="" width={16} height={16} className='block md:hidden' />
              <Image src="/img/img37.png" alt="" width={24} height={24} className='hidden md:block' />
            </Link>
          ))
        }
      </div>
    </div>
  )
}
