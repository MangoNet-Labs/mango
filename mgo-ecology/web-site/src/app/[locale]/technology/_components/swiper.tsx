"use client"
import { NetworkType } from '@/interface';
import { Carousel } from 'antd';
import { useRef } from 'react'

export default function swiper({ System }: { System: NetworkType[] }) {
  const carouselRef = useRef<any>(null)

  const prevSwiper = () => {
    if (carouselRef.current) {
      carouselRef.current.prev()
    }
  }
  const nextSwiper = () => {
    if (carouselRef.current) {
      carouselRef.current.next()
    }
  }

  return (
    <div className="technology-style">
      <div className="tec-grid2">
        <div className="column-center" >
          <img src="/img/img77.png" className="w-4 md:w-[80px] be_hover rounded-lg" alt="" onClick={prevSwiper} />
        </div>
        <div className="overflow-hidden">
          <Carousel dots={false} ref={carouselRef} infinite={false}>
            {
              System.map(item => (
                <div key={item.ID}>
                  <div className='pb-[10px] md:pb-[50px] text-grayb2 md:text-white text-[12px] md:text-xl text-center'>{item.introduction}</div>
                  <div className='tec-swi'>
                    <div className='tec-swi-top'>{item.title}</div>
                    <div className='p-10 text-white' dangerouslySetInnerHTML={{ __html: item.content }}></div>
                  </div>
                </div>
              ))
            }
          </Carousel>
        </div>
        <div className="column-center">
          <img src="/img/img78.png" className="w-4 md:w-[80px] be_hover rounded-lg" alt="" onClick={nextSwiper} />
        </div>
      </div>
    </div>
  )
}
