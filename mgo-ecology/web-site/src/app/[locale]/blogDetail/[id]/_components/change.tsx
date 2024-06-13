"use client"
import '@/styles/blogdetail.scss'
import { useRouter } from 'next/navigation'
import { BlogDetailType } from '@/interface';
import { LeftOutlined, RightOutlined } from '@ant-design/icons'

export default function change({ blogDetail }: { blogDetail: BlogDetailType }) {
  const router = useRouter()

  return (
    <div className='blogdetail-style'>
      <div className='b2b-box1 between-center'>
        {
          blogDetail.previous ?
            <div className="start-center b2b-change be_pointer" onClick={() => router.replace(`/blogDetail/${blogDetail?.previous?.ID}`)}>
              <LeftOutlined style={{
                color: '#fff',
                fontSize: '16px'
              }} />
              <div className="b2b-text2 ellipsis">{blogDetail?.previous.title}</div>
            </div> :
            <div></div>
        }
        {
          blogDetail.next ?
            <div className="end-center b2b-change be_pointer" onClick={() => router.replace(`/blogDetail/${blogDetail?.next?.ID}`)}>
              <div className="b2b-text2 ellipsis">{blogDetail?.next.title}
              </div>
              <RightOutlined style={{
                color: '#fff',
                fontSize: '16px'
              }} />
            </div> :
            <div></div>
        }
      </div>
    </div>
  )
}
